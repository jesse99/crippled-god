use super::core::*;
use super::level;

pub struct Player {
	loc: Point,
	ready: Time,
}

impl Player {
	pub fn new() -> Player {
		Player {
			loc: Point::origin(),
			ready: Time::zero(),
		}
	}

	pub fn ready_time(&self) -> Time {
		self.ready
	}

	pub fn on_event(&mut self, event: &Event, queued: &mut QueuedEvents, level: &level::Level) {
		match event {
			Event::NewLevel => {
				let loc = find_initial_loc(level);
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

fn find_initial_loc(_level: &level::Level) -> Point {
	Point::new(4, 2)
}
