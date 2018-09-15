use super::colors::*;
use super::console::*;
use super::map::*;
use super::persist::*;
use super::prompt::*;
use backend;
use std;
use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

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

pub fn run(config_file: Result<String, String>, seed: u64) {
	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);
	stdout.flush().unwrap();

	let mut game = create_game(&mut stdout, config_file, seed);

	let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	let terminal_size = backend::Size::new(width as i32, height as i32);

	let mut key_iter = stdin.keys();
	loop {
		render_game(terminal_size, &mut stdout, &mut game);
		if game.players_time_slice() {
			if let Some(c) = key_iter.next() {
				let cc = c.unwrap();
				let key = map_key(cc);
				if !game.handle_key(key) {
					warn!("user pressed {:?}", cc);
					let _ = write!(stdout, "\x07");
					stdout.flush().unwrap();
				}

				if !game.running() {
					break;
				}
			}
		}
	}
	save_game(&mut stdout, &game);
	restore();
}

fn create_game(
	stdout: &mut RawTerminal,
	config_file: Result<String, String>,
	seed: u64,
) -> backend::Game {
	if has_saved_game() {
		let choices = vec![
			Choice::new2(
				termion::event::Key::Char('y'),
				termion::event::Key::Char('\n'),
				"Load the saved game?",
			),
			Choice::new2(
				termion::event::Key::Char('n'),
				termion::event::Key::Esc,
				"Don’t load the saved game (and overwrite the old game with a new game).",
			),
		];
		if prompt(stdout, choices) == 0 {
			match load_game() {
				Ok(game) => {
					return game;
				}
				Err(err) => {
					let bg = to_termion(Color::Black);
					let fg = to_termion(Color::Red);
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

	info!("new game with seed {}", seed);
	backend::Game::new(config_file, seed)
}

fn render_game(terminal_size: backend::Size, stdout: &mut RawTerminal, game: &mut backend::Game) {
	render_map(terminal_size, stdout, game);
	render_console(terminal_size, stdout, &game);
	stdout.flush().unwrap();
}

fn map_key(key: termion::event::Key) -> backend::Key {
	match key {
		termion::event::Key::Left => backend::Key::LeftArrow,
		termion::event::Key::Right => backend::Key::RightArrow,
		termion::event::Key::Up => backend::Key::UpArrow,
		termion::event::Key::Down => backend::Key::DownArrow,
		termion::event::Key::Char(ch) => backend::Key::Char(ch),
		_ => backend::Key::Char('\x04'),
	}
}
