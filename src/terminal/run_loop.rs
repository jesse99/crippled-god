use super::console::*;
use super::map::*;
use backend;
use std;
use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn run(config_file: Option<String>, seed: usize) {
	info!("running terminal with seed {}", seed);
	let mut game = backend::Game::new(config_file, seed);

	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);

	let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	let terminal_size = backend::Size::new(width as i32, height as i32);

	stdout.flush().unwrap();
	render_game(terminal_size, &mut stdout, &mut game);
	for c in stdin.keys() {
		let key = map_key(c.unwrap());
		if !game.handle_key(key) {
			let _ = write!(stdout, "\x07");
		}

		if !game.running() {
			break;
		}
		render_game(terminal_size, &mut stdout, &mut game);
	}

	let _ = write!(
		stdout,
		"{}{}{}",
		termion::cursor::Restore,
		termion::cursor::Show,
		termion::cursor::Goto(1, 1)
	);
	stdout.flush().unwrap();
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
