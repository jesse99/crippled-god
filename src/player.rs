use super::core::*;
use super::level::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	// DeltaEast,
	// DeltaNorth,
	// DeltaNorthEast,
	// DeltaNorthWest,
	// DeltaSouth,
	// DeltaSouthEast,
	// DeltaSouthWest,
	// DeltaWest,
	Quit,
}

pub struct Player {
	loc: Point,
}

impl Player {
	pub fn new() -> Player {
		Player {
			loc: Point::origin(),
		}
	}

	pub fn on_event(&mut self, event: &Event, queued: &mut QueuedEvents, level: &Level) {
		match event {
			Event::NewLevel => {
				let loc = find_initial_loc(&level);
				queued.push_back(Event::SetPlayer(loc));
			}
			Event::SetPlayer(loc) => {
				// TODO: should have an assert here (or maybe in Level) that loc is sane
				self.loc = *loc;
			}
			_ => (),
		}
	}
}

fn find_initial_loc(_level: &Level) -> Point {
	Point::new(4, 2)
}
