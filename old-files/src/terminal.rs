mod colors;
mod console;
mod map;
mod persist;
mod prompt;
mod view;

use std::io::Write;
use std::panic;
use std::process::Command;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::backend::{self, Game, PlayerAction, Size};
use slog::Logger;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn run(config_path: Option<String>, root_logger: &Logger, seed: u64) {
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);
	stdout.flush().unwrap();

	let old_hook = panic::take_hook();
	panic::set_hook(Box::new(move |arg| {
		restore();
		let mut stdout = std::io::stdout();
		let _ = write!(stdout, "{}", termion::clear::All);
		let _ = Command::new("reset").output(); // new line mode isn't reset w/o this
		old_hook(arg);
	}));

	let mut game = create_game(&mut stdout, config_path, root_logger, seed);

	let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	let terminal_size = Size::new(i32::from(width), i32::from(height));

	let mut key_iter = stdin.keys();
	loop {
		game.execute_others();
		render_game(terminal_size, &mut stdout, &mut game);

		if let Some(c) = key_iter.next() {
			let cc = c.unwrap();
			if let Some(action) = map_player_action(cc) {
				game.dispatch_action(action);
				if !game.running() {
					break;
				}
			} else if let Some(action) = map_game_action(cc) {
				dispatch_game_action(&mut stdout, &mut game, action);
				if !game.running() {
					break;
				}
			} else {
				warn!(root_logger, "user pressed"; "key" => format!("{:?}", cc));
				let _ = write!(stdout, "\x07");
				stdout.flush().unwrap();
			}
		}
	}
	// save_game(&mut stdout, &game);
	restore();
}

fn restore() {
	let mut stdout = std::io::stdout();
	let _ = write!(
		stdout,
		"{}{}{}{}",
		termion::style::Reset,
		termion::cursor::Restore,
		termion::cursor::Show,
		termion::cursor::Goto(1, 1)
	);
	stdout.flush().unwrap();
}

fn create_game(stdout: &mut RawTerminal, config_path: Option<String>, root_logger: &Logger, seed: u64) -> Game {
	if persist::has_saved_game() {
		let choices = vec![
			prompt::Choice::new2(
				termion::event::Key::Char('y'),
				termion::event::Key::Char('\n'),
				"Load the saved game?",
			),
			prompt::Choice::new2(
				termion::event::Key::Char('n'),
				termion::event::Key::Esc,
				"Don’t load the saved game (and overwrite the old game with a new game).",
			),
		];
		if prompt::prompt(stdout, &choices) == 0 {
			match persist::load_game(root_logger) {
				Ok(game) => {
					return game;
				}
				Err(err) => {
					let bg = colors::to_termion(colors::Color::Black);
					let fg = colors::to_termion(colors::Color::Red);
					let _ = write!(
						stdout,
						"{}{}{}{}couldn't load the game: {}",
						termion::color::Bg(bg),
						termion::clear::All,
						termion::cursor::Goto(1, 1),
						termion::color::Fg(fg),
						err,
					);
					stdout.flush().unwrap();
				}
			}
		}
	}

	info!(root_logger, "new game"; "config_path" => &config_path, "seed" => seed);
	Game::new(config_path, root_logger, seed)
}

fn render_game(terminal_size: Size, stdout: &mut RawTerminal, game: &mut Game) {
	map::render_map(terminal_size, stdout, game);
	console::render_console(terminal_size, stdout, game);
	stdout.flush().unwrap();
}

enum GameAction {
	SaveGame,
}

fn dispatch_game_action(stdout: &mut RawTerminal, game: &mut Game, action: GameAction) {
	match action {
		GameAction::SaveGame => persist::save_game(game),
	}
}

fn map_game_action(key: termion::event::Key) -> Option<GameAction> {
	match key {
		termion::event::Key::Ctrl('s') => Some(GameAction::SaveGame),
		_ => None,
	}
}

fn map_player_action(key: termion::event::Key) -> Option<PlayerAction> {
	match key {
		termion::event::Key::Left => Some(PlayerAction::DeltaWest),
		termion::event::Key::Right => Some(PlayerAction::DeltaEast),
		termion::event::Key::Up => Some(PlayerAction::DeltaNorth),
		termion::event::Key::Down => Some(PlayerAction::DeltaSouth),
		termion::event::Key::Char('1') => Some(PlayerAction::DeltaSouthWest),
		termion::event::Key::Char('2') => Some(PlayerAction::DeltaSouth),
		termion::event::Key::Char('3') => Some(PlayerAction::DeltaSouthEast),
		termion::event::Key::Char('4') => Some(PlayerAction::DeltaWest),
		termion::event::Key::Char('6') => Some(PlayerAction::DeltaEast),
		termion::event::Key::Char('7') => Some(PlayerAction::DeltaNorthWest),
		termion::event::Key::Char('8') => Some(PlayerAction::DeltaNorth),
		termion::event::Key::Char('9') => Some(PlayerAction::DeltaNorthEast),
		termion::event::Key::Char('q') => Some(PlayerAction::Quit),
		_ => None,
	}
}
