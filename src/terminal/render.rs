use super::super::core::*;
use super::super::level::*;
use super::super::player::*;
use super::view::*;
use std::io::Write;
use termion;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn render_level(
	stdout: &mut RawTerminal,
	level: &mut Level,
	player: &Player,
	terminal_size: Size,
) {
	let tiles = level.tiles(terminal_size, player);

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
