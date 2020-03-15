use super::super::core::*;
use super::super::level::*;
use super::super::player::*;
use super::view::*;
use std::io::Write;
use termion;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn render_level(stdout: &mut RawTerminal, level: &Level, player: &Player, terminal_size: Size) {
	for y in 0..terminal_size.height {
		for x in 0..terminal_size.width {
			let loc = Point::new(x, y);
			if level.is_valid(loc) {
				let view = View::new(level, player, loc);

				let h = (x + 1) as u16; // termion is 1-based
				let v = (y + 1) as u16;
				let _ = write!(
					stdout,
					"{}{}{}{}",
					termion::cursor::Goto(h, v),
					termion::color::Bg(view.bg),
					termion::color::Fg(view.fg),
					view.symbol
				);
			}
		}
	}

	// let map_size = Size::new(
	// 	terminal_size.width,
	// 	terminal_size.height - 0,
	// 	//        terminal_size.height - game.config.terminal.num_lines,
	// );
	// let tiles = game.tiles(map_size);

	// for (loc, tile) in tiles.iter() {
	// 	let view = View::new(game, tile);
	// 	let x = (loc.x + 1) as u16; // termion is 1-based
	// 	let y = (loc.y + 1) as u16;
	// 	let _ = write!(
	// 		stdout,
	// 		"{}{}{}{}",
	// 		termion::cursor::Goto(x, y),
	// 		termion::color::Bg(view.bg),
	// 		termion::color::Fg(view.fg),
	// 		view.symbol
	// 	);
	// }
}
