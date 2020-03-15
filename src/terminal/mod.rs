mod color;
mod render;
mod view;

use super::core::*;
use super::level::*;
use super::player::*;
use render::*;
use slog::Logger;
use std::io::Write;
use std::panic::{set_hook, take_hook};
use std::process;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub struct Terminal {
	logger: Logger,
	stdout: RawTerminal,
	ready: Time, // this is basically the player ready time
}

impl Terminal {
	pub fn new(root_logger: &Logger) -> Terminal {
		Terminal {
			logger: root_logger.new(o!()),
			stdout: setup_terminal(),
			ready: Time::from_seconds(1),
		}
	}

	pub fn ready_time(&self) -> Time {
		self.ready
	}

	pub fn on_event(
		&mut self,
		event: &Event,
		_queued: &mut QueuedEvents,
		level: &Level,
		player: &mut Player,
	) -> bool {
		// TODO:
		//    map it to an action
		//    dispatch it to a handler, player actions will need to return a duration
		if let Event::AdvanceTime(time) = event {
			assert!(*time <= self.ready);
			if *time == self.ready {
				let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
				let terminal_size = Size::new(i32::from(width), i32::from(height));
				render_level(&mut self.stdout, level, player, terminal_size);
				self.stdout.flush().unwrap();

				let stdin = std::io::stdin();
				let mut key_iter = stdin.keys(); // TODO: may want to make this a field
				if let Some(c) = key_iter.next() {
					let cc = c.unwrap();
					debug!(self.logger, "handling"; "key" => ?cc);
					if let Some(action) = key_to_action(cc) {
						if !player.on_action(action) {
							match on_game_action(action) {
								GameResult::NotRunning => {
									restore_terminal();
									return false;
								}
								GameResult::Ignored => panic!("Didn't handle action {:?}", action),
							}
						}
					// }
					// } else if let Some(action) = map_game_action(cc) {
					// 	dispatch_game_action(&mut self.stdout, &mut game, action);
					// 	if !game.running() {
					// 		break;
					// 	}
					// } else {
					// 	warn!(self.logger, "user pressed"; "key" => format!("{:?}", cc));
					// 	let _ = write!(self.stdout, "\x07");
					// 	self.stdout.flush().unwrap();
					} else {
						// Note that we don't advance ready time for bad keys (or
						// game actions). This means that the event loop will queue
						// up another AdvanceTime event for the same time but that
						// doesn't do any harm and simplifies our logic.
						trace!(self.logger, "ignoring"; "key" => ?cc);
					}
				}
			}
		}
		true
	}
}

enum GameResult {
	NotRunning,
	Ignored,
}

fn on_game_action(action: PlayerAction) -> GameResult {
	if let PlayerAction::Quit = action {
		GameResult::NotRunning
	} else {
		GameResult::Ignored
	}
}

fn setup_terminal() -> RawTerminal {
	let mut stdout = std::io::stdout().into_raw_mode().unwrap();
	let _ = write!(stdout, "{}{}", termion::cursor::Hide, termion::clear::All);
	stdout.flush().unwrap();
	let old_hook = take_hook();
	set_hook(Box::new(move |arg| {
		restore_terminal();
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
	let _ = write!(stdout, "{}", termion::clear::All);
	stdout.flush().unwrap();

	let _ = process::Command::new("reset").output(); // new line mode isn't reset w/o this
}

fn key_to_action(key: termion::event::Key) -> Option<PlayerAction> {
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
