use super::rng::*;
use super::scheduled::*;
use super::*;
use std::f32;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
	character: Character,
	ready_time: Time,
	hp: i32,
}

impl Player {
	pub fn new(character: Character) -> Player {
		let ready_time = Time::zero();
		let hp = 100;
		Player {
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

	/// Used for normal movement, i.e. not something like a teleport.
	pub fn on_moved(&mut self, terrain: Terrain, dx: i32, dy: i32) {
		let delay = (attributes(self.character).movement_delay)(terrain);
		let delay = if dx != 0 && dy != 0 {
			1.414 * delay
		} else {
			delay
		};
		assert!(delay > 0.0);
		self.ready_time = self.ready_time + delay as i64;
		// info!(
		// 	"player moved by {},{} and will be ready at {}",
		// 	dx, dy, self.ready_time
		// );
	}
}

impl Scheduled for Player {
	fn ready_time(&self) -> Time {
		self.ready_time
	}

	fn execute(&mut self, _level: &mut Level, _loc: Location, _rng: &mut RNG) -> Option<Location> {
		assert!(false, "execute shouldn't be called on the player");
		None
	}
}
