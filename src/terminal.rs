mod colors;
mod map;
mod view;

use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::backend::{self, Game, PlayerAction, Size};
use slog::Logger;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn run(root_logger: &Logger, seed: u64) {
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);
	stdout.flush().unwrap();

	let mut game = create_game(root_logger, seed);

	let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	let terminal_size = Size::new(i32::from(width), i32::from(height));

	let mut key_iter = stdin.keys();
	loop {
		render_game(terminal_size, &mut stdout, &mut game);
		// if game.players_time_slice() {
		if let Some(c) = key_iter.next() {
			let cc = c.unwrap();
			if let Some(action) = map_action(cc) {
				game.dispatch_action(action);
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
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(
		stdout,
		"{}{}{}",
		termion::cursor::Restore,
		termion::cursor::Show,
		termion::cursor::Goto(1, 1)
	);
	stdout.flush().unwrap();
}

fn create_game(root_logger: &Logger, seed: u64) -> Game {
	Game::new(root_logger, seed)
}

fn render_game(terminal_size: Size, stdout: &mut RawTerminal, game: &mut Game) {
	map::render_map(terminal_size, stdout, game);
	// render_console(terminal_size, stdout, &game);
	stdout.flush().unwrap();
}

fn map_action(key: termion::event::Key) -> Option<PlayerAction> {
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
