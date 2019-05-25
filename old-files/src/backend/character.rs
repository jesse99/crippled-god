use super::rng::*;
use super::scheduled::*;
use super::*;
use std::f32;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CharName {
	Ay,       // giant wolf
	Bhederin, // large herbivore
	Human,
	// Toblakai,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Character {
	name: CharName,
	label: String,
	ready_time: Time,
	hp: i32,
}

// TODO: Attributes instead of Stats
// Attributes includes name and movement speed
impl Character {
	pub fn new_npc(name: CharName) -> Character {
		let label = char_name_to_label(name);
		let ready_time = Time::zero();
		let hp = 100;
		Character {
			name,
			label,
			ready_time,
			hp,
		}
	}

	pub fn new_player(name: CharName) -> Character {
		let label = "you".to_string();
		let ready_time = Time::zero();
		let hp = 100;
		Character {
			name,
			label,
			ready_time,
			hp,
		}
	}

	pub fn name(&self) -> CharName {
		self.name
	}

	pub fn label(&self) -> &str {
		&self.label
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.get_terrain(loc);
		let delay = (attributes(self.name).movement_delay)(terrain);
		delay < f32::INFINITY && level.has_char(loc)
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
			self.on_moved(terrain, delta.x, delta.y);
		} else {
			self.ready_time = self.ready_time + 1; // TODO: might want to sleep for a random time
		}
		loc + delta
	}

	/// Used for normal movement, i.e. not something like a teleport.
	pub fn on_moved(&mut self, terrain: Terrain, dx: i32, dy: i32) {
		let delay = (attributes(self.name).movement_delay)(terrain);
		let delay = if dx != 0 && dy != 0 {
			1.414 * delay
		} else {
			delay
		};
		assert!(delay > 0.0);
		self.ready_time = self.ready_time + delay as i64;
	}
}

impl Scheduled for Character {
	fn ready_time(&self) -> Time {
		self.ready_time
	}

	fn execute(&mut self, level: &mut Level, loc: Location, rng: &mut RNG) -> Option<Location> {
		let old_time = self.ready_time();
		let result = match self.name {
			CharName::Ay => self.execute_aggressive(level, loc, rng),
			CharName::Bhederin => self.execute_skittish(level, loc, rng),
			CharName::Human => self.execute_aggressive(level, loc, rng),
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

fn char_name_to_label(name: CharName) -> String {
	match name {
		CharName::Ay => "ay",
		CharName::Bhederin => "bhederin",
		CharName::Human => "human",
	}.to_string()
}
