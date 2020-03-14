mod color;
mod render;
mod view;

use super::core::*;
use super::level::*;
use super::player::*;
use render::*;
use std::io::Write;
use std::panic::{set_hook, take_hook};
use std::process;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub struct Terminal {
	stdout: RawTerminal,
	ready: Time, // this is basically the player ready time
}

impl Terminal {
	pub fn new() -> Terminal {
		Terminal {
			stdout: setup_terminal(),
			ready: Time::zero(),
		}
	}

	pub fn ready_time(&self) -> Time {
		self.ready
	}

	pub fn on_event(&mut self, event: &Event, queued: &mut QueuedEvents, level: &Level) -> bool {
		// TODO:
		// on AdvanceTime(time)
		// assert time <= self.ready
		// if time == self.ready then
		//    render map
		//    get a key stroke
		//    map it to an action
		//    dispatch it to a handler, player actions will need to return a duration
		//    this stuff should only be done when the player is ready
		let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
		let terminal_size = Size::new(i32::from(width), i32::from(height));
		render_level(&mut self.stdout, level, terminal_size);

		let stdin = std::io::stdin();
		let mut key_iter = stdin.keys();
		if let Some(c) = key_iter.next() {
			let cc = c.unwrap();
			if let Some(action) = map_player_action(cc) {
				// game.dispatch_action(action);
				if let PlayerAction::Quit = action {
					return false;
				}
				// } else if let Some(action) = map_game_action(cc) {
				// 	dispatch_game_action(&mut self.stdout, &mut game, action);
				// 	if !game.running() {
				// 		break;
				// 	}
				// } else {
				// 	warn!(root_logger, "user pressed"; "key" => format!("{:?}", cc));
				// 	let _ = write!(self.stdout, "\x07");
				// 	self.stdout.flush().unwrap();
			}
		}
		true
	}
}

fn setup_terminal() -> RawTerminal {
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);
	stdout.flush().unwrap();
	let old_hook = take_hook();
	set_hook(Box::new(move |arg| {
		restore_terminal();
		let mut stdout = std::io::stdout();
		let _ = write!(stdout, "{}", termion::clear::All);
		let _ = process::Command::new("reset").output(); // new line mode isn't reset w/o this
		old_hook(arg);
	}));
	stdout
}

fn restore_terminal() {
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

fn map_player_action(key: termion::event::Key) -> Option<PlayerAction> {
	match key {
		// termion::event::Key::Left => Some(PlayerAction::DeltaWest),
		// termion::event::Key::Right => Some(PlayerAction::DeltaEast),
		// termion::event::Key::Up => Some(PlayerAction::DeltaNorth),
		// termion::event::Key::Down => Some(PlayerAction::DeltaSouth),
		// termion::event::Key::Char('1') => Some(PlayerAction::DeltaSouthWest),
		// termion::event::Key::Char('2') => Some(PlayerAction::DeltaSouth),
		// termion::event::Key::Char('3') => Some(PlayerAction::DeltaSouthEast),
		// termion::event::Key::Char('4') => Some(PlayerAction::DeltaWest),
		// termion::event::Key::Char('6') => Some(PlayerAction::DeltaEast),
		// termion::event::Key::Char('7') => Some(PlayerAction::DeltaNorthWest),
		// termion::event::Key::Char('8') => Some(PlayerAction::DeltaNorth),
		// termion::event::Key::Char('9') => Some(PlayerAction::DeltaNorthEast),
		termion::event::Key::Char('q') => Some(PlayerAction::Quit),
		_ => None,
	}
}
