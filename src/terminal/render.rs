use super::super::core::*;
use super::super::level::*;
use super::view::*;
use std::io::Write;
use termion;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn render_level(stdout: &mut RawTerminal, store: &mut Store, terminal_size: Size) {
	let seen = get_last_seen(store, terminal_size);

	for (loc, cell) in seen.iter() {
		let view = View::new(store, cell);
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
