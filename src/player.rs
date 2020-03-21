use super::character::*;
use super::core::*;
use super::level::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	DeltaEast,
	DeltaNorth,
	DeltaNorthEast,
	DeltaNorthWest,
	DeltaSouth,
	DeltaSouthEast,
	DeltaSouthWest,
	DeltaWest,
	Quit,
}

pub enum PlayerActionResult {
	Acted(Duration),

	/// Action doesn't apply to the player.
	Ignored,

	/// Player tried to do something but was unable to, e.g. moving into a wall.
	Error,
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

	pub fn loc(&self) -> Point {
		self.loc
	}

	pub fn on_event(
		&mut self,
		rng: &mut SmallRng,
		event: &Event,
		queued: &mut QueuedEvents,
		level: &Level,
	) {
		match event {
			Event::NewLevel => {
				let loc = find_initial_loc(rng, &level).unwrap();
				queued.push_back(Event::SetPlayer(loc));
			}
			Event::SetPlayer(loc) => {
				// TODO: should have an assert here (or maybe in Level) that loc is sane
				self.loc = *loc;
			}
			_ => (),
		}
	}

	pub fn on_action(&mut self, action: PlayerAction, level: &Level) -> PlayerActionResult {
		match action {
			PlayerAction::DeltaEast => self.move_by(level, 1, 0),
			PlayerAction::DeltaNorth => self.move_by(level, 0, -1),
			PlayerAction::DeltaNorthEast => self.move_by(level, 1, -1),
			PlayerAction::DeltaNorthWest => self.move_by(level, -1, -1),
			PlayerAction::DeltaSouth => self.move_by(level, 0, 1),
			PlayerAction::DeltaSouthEast => self.move_by(level, 1, 1),
			PlayerAction::DeltaSouthWest => self.move_by(level, -1, 1),
			PlayerAction::DeltaWest => self.move_by(level, -1, 0),
			_ => PlayerActionResult::Ignored,
		}
	}

	fn move_by(&mut self, level: &Level, dx: i32, dy: i32) -> PlayerActionResult {
		assert!(dx != 0 || dy != 0);

		let new_loc = Point {
			x: self.loc.x + dx,
			y: self.loc.y + dy,
		};
		if can_move_to(level, new_loc) {
			self.loc = new_loc;
			if dx != 0 && dy != 0 {
				PlayerActionResult::Acted(Duration::from_secs(1.4 * 2.0))
			} else {
				PlayerActionResult::Acted(Duration::from_secs(2.0))
			}
		} else {
			PlayerActionResult::Error // TODO: should we include a reason?
		}
	}
}

fn find_initial_loc(rng: &mut SmallRng, level: &Level) -> Option<Point> {
	let size = level.size();
	let mut indexes: Vec<i32> = (0..size.width * size.height).collect();
	indexes.shuffle(rng);

	for i in &indexes {
		let x = i % size.width;
		let y = i / size.width;
		let loc = Point::new(x, y);
		let cell = level.get(loc);
		if let Terrain::Ground = cell {
			// if cell.character.is_none() && predicate(cell) {
			return Some(loc);
		}
	}
	None
}
