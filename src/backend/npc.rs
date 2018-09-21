use super::rng::*;
use super::scheduled::*;
use super::*;
use std::f32;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NPC {
	character: Character,
	ready_time: Time,
	hp: i32,
}

// TODO: Attributes instead of Stats
// Attributes includes name and movement speed
impl NPC {
	pub fn new(character: Character) -> NPC {
		let ready_time = Time::zero();
		let hp = 100;
		NPC {
			character,
			ready_time,
			hp,
		}
	}

	pub fn character(&self) -> Character {
		self.character
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.get_terrain(loc);
		let delay = (attributes(self.character).movement_delay)(terrain);
		delay < f32::INFINITY && level.empty(loc)
	}

	fn execute_aggressive(
		&mut self,
		level: &mut Level,
		loc: Location,
		rng: &mut RNG,
	) -> Option<Location> {
		let ploc = level.player_loc();
		let left_is_better = |lhs, rhs| {
			let d1 = ploc.distance(lhs);
			let d2 = ploc.distance(rhs);
			d1 > 0.0 && d1 < d2
		};
		Some(self.move_relative_to_player(level, loc, left_is_better, rng))
	}

	fn execute_skittish(
		&mut self,
		level: &mut Level,
		loc: Location,
		rng: &mut RNG,
	) -> Option<Location> {
		let ploc = level.player_loc();
		let left_is_better = |lhs, rhs| {
			let d1 = ploc.distance(lhs);
			let d2 = ploc.distance(rhs);
			d1 > d2
		};
		Some(self.move_relative_to_player(level, loc, left_is_better, rng))
	}

	fn move_relative_to_player<F: Fn(Location, Location) -> bool>(
		&mut self,
		level: &mut Level,
		loc: Location,
		left_is_better: F,
		rng: &mut RNG,
	) -> Location {
		let mut delta = Location::zero();
		if level.is_visible(loc, level.player_loc()) {
			let mut deltas = vec![
				Location::new(-1, -1),
				Location::new(-1, 0),
				Location::new(-1, 1),
				Location::new(0, -1),
				Location::new(0, 0),
				Location::new(0, 1),
				Location::new(1, -1),
				Location::new(1, 0),
				Location::new(1, 1),
			];

			rng.shuffle(&mut deltas);
			for candidate in deltas {
				if self.can_move_to(level, loc + candidate)
					&& left_is_better(loc + candidate, loc + delta)
				{
					delta = candidate;
				}
			}
		}
		if delta != Location::zero() {
			let terrain = level.get_terrain(loc + delta);
			let delay = (attributes(self.character).movement_delay)(terrain);
			let delay = if delta.x != 0 && delta.y != 0 {
				1.414 * delay
			} else {
				delay
			};
			assert!(delay != f32::INFINITY);
			assert!(delay >= 1.0, "npc delay is {}", delay);
			self.ready_time = self.ready_time + delay as i64;
		} else {
			self.ready_time = self.ready_time + 1; // TODO: might want to sleep for a random time
		}
		loc + delta
	}
}

impl Scheduled for NPC {
	fn ready_time(&self) -> Time {
		self.ready_time
	}

	fn execute(&mut self, level: &mut Level, loc: Location, rng: &mut RNG) -> Option<Location> {
		let old_time = self.ready_time();
		let result = match self.character {
			Character::Ay => self.execute_aggressive(level, loc, rng),
			Character::Bhederin => self.execute_skittish(level, loc, rng),
			Character::Human => self.execute_aggressive(level, loc, rng),
		};
		assert!(
			self.ready_time() > old_time,
			"ready_time={} old_time={}",
			self.ready_time(),
			old_time
		);
		result
	}
}
