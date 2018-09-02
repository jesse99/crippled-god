use backend;
use std;
use std::io::Write;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use super::tile::Tile;

pub type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn run(seed: usize) {
	info!("running terminal with seed {}", seed);
	let mut level = backend::Level::new(seed);

	let stdin = std::io::stdin();
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);

	// TODO: use terminal size to compute screen size
	// let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
	// println!("width = {}, height = {}", width, height);

	stdout.flush().unwrap();
	render_map(&mut stdout, &mut level);
	for c in stdin.keys() {
		match c.unwrap() {
			termion::event::Key::Char('q') => break,
			termion::event::Key::Left => move_player(&mut stdout, &mut level, -1, 0),
			termion::event::Key::Right => move_player(&mut stdout, &mut level, 1, 0),
			termion::event::Key::Up => move_player(&mut stdout, &mut level, 0, -1),
			termion::event::Key::Down => move_player(&mut stdout, &mut level, 0, 1),
			// termion::event::Key::Ctrl('r') => map = create_map(&mut rng),
			_ => {
				let _ = write!(stdout, "\x07");
			}
		};
		render_map(&mut stdout, &mut level);
		stdout.flush().unwrap();
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

fn move_player(stdout: &mut RawTerminal, level: &mut backend::Level, dx: i32, dy: i32) {
	let p = level.player.loc;
	let loc = backend::Location::new(p.x + dx, p.y + dy);
	if level.player.can_move_to(level, loc) {
		level.move_player(loc);
	} else {
		let _ = write!(stdout, "\x07");
	}
}

fn render_map(stdout: &mut RawTerminal, level: &mut backend::Level) {
	let screen_size = backend::Size::new(40, 30);
	let cells = level.get_cells(screen_size);

	for (loc, cell) in cells.iter() {
		let tile = Tile::new(cell);
		let x = (loc.x + 1) as u16; // termion is 1-based
		let y = (loc.y + 1) as u16;
		let _ = write!(
			stdout,
			"{}{}{}{}",
			termion::cursor::Goto(x, y),
			termion::color::Bg(tile.bg),
			termion::color::Fg(tile.fg),
			tile.symbol
		);
	}
	stdout.flush().unwrap();
}
