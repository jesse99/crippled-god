use backend;
use std;
use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::console::*;
use super::map::*;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn run(seed: usize) {
	info!("running terminal with seed {}", seed);
	let mut game = backend::Game::new(seed);

	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);

	let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	let terminal_size = backend::Size::new(width as i32, height as i32);

	stdout.flush().unwrap();
	render_game(terminal_size, &mut stdout, &mut game);
	for c in stdin.keys() {
		match c.unwrap() {
			termion::event::Key::Char('q') => break,
			termion::event::Key::Left => move_player(&mut stdout, &mut game, -1, 0),
			termion::event::Key::Right => move_player(&mut stdout, &mut game, 1, 0),
			termion::event::Key::Up => move_player(&mut stdout, &mut game, 0, -1),
			termion::event::Key::Down => move_player(&mut stdout, &mut game, 0, 1),
			// termion::event::Key::Ctrl('r') => map = create_map(&mut rng),
			_ => {
				let _ = write!(stdout, "\x07");
			}
		};
		render_game(terminal_size, &mut stdout, &mut game);
	}

	let _ = write!(
		stdout,
		"{}{}{}",
		termion::cursor::Restore,
		termion::cursor::Show,
		termion::cursor::Goto(1, 20)
	);
	stdout.flush().unwrap();
}

fn render_game(terminal_size: backend::Size, stdout: &mut RawTerminal, game: &mut backend::Game) {
	render_map(terminal_size, stdout, game);
	render_console(terminal_size, stdout, &game);
	stdout.flush().unwrap();
}

fn move_player(stdout: &mut RawTerminal, game: &mut backend::Game, dx: i32, dy: i32) {
	let p = game.level.player_loc;
	let loc = backend::Location::new(p.x + dx, p.y + dy);
	if game.player.can_move_to(&game.level, loc) {
		game.level.move_player(&game.player, loc);
	} else {
		let _ = write!(stdout, "\x07");
	}
}
