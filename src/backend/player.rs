use super::rng::*;
use super::scheduled::*;
use super::*;
use std::f32;

pub const BASE_MOVEMENT_SPEED: f32 = 5.0;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Race {
	Human,
	// Toblakai,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Player {
	race: Race,
	ready_time: Time,
}

impl Player {
	pub fn new(race: Race) -> Player {
		let ready_time = Time::zero();
		Player { race, ready_time }
	}

	pub fn race(&self) -> Race {
		self.race
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.get_terrain(loc);
		let delay = self.race.delay(terrain);
		delay < f32::INFINITY && level.empty(loc)
	}

	/// Used for normal movement, i.e. not something like a teleport.
	pub fn on_moved(&mut self, terrain: Terrain, dx: i32, dy: i32) {
		let delay = self.race.delay(terrain);
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

impl MovementDelay for Race {
	fn delay(&self, terrain: Terrain) -> f32 {
		match self {
			Race::Human => match terrain {
				Terrain::Blank => {
					assert!(false); // blank should only be used for rendering
					f32::INFINITY
				}
				Terrain::DeepWater => f32::INFINITY,
				Terrain::Ground => BASE_MOVEMENT_SPEED,
				Terrain::ShallowWater => 0.9 * BASE_MOVEMENT_SPEED,
				Terrain::Wall => f32::INFINITY,
			},
			// Race::Toblakai => match terrain {
			// 	Terrain::Blank => {
			// 		assert!(false); // blank should only be used for rendering
			// 		f32::INFINITY
			// 	}
			// 	Terrain::DeepWater => f32::INFINITY,
			// 	Terrain::Ground => 1.1*BASE_MOVEMENT_SPEED,
			// 	Terrain::ShallowWater => 1.1*BASE_MOVEMENT_SPEED,
			// 	Terrain::Wall => f32::INFINITY,
			// },
		}
	}
}

impl MovementDelay for Player {
	fn delay(&self, terrain: Terrain) -> f32 {
		self.race.delay(terrain)
	}
}
