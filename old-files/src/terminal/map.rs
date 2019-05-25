//! The part of the terminal that shows the terrain, the position of the player, etc.
use backend;
use std;
use std::io::Write;
use termion;

// use super::console::*;
use super::view::*;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn render_map(
	terminal_size: backend::Size,
	stdout: &mut RawTerminal,
	game: &mut backend::Game,
) {
	let map_size = backend::Size::new(
		terminal_size.width,
		terminal_size.height - game.config().terminal.num_lines,
	);
	let tiles = game.get_tiles(map_size);

	for (loc, tile) in tiles.iter() {
		let view = View::new(tile);
		let x = (loc.x + 1) as u16; // termion is 1-based
		let y = (loc.y + 1) as u16;
		let _ = write!(
			stdout,
			"{}{}{}{}",
			termion::cursor::Goto(x, y),
			termion::color::Bg(view.bg),
			termion::color::Fg(view.fg),
			view.symbol
		);
	}
}
