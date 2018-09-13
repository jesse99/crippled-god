use super::scheduled::*;
use super::*;

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
	const MOVEMENT_SPEED: i64 = 5;

	pub fn new(race: Race) -> Player {
		let ready_time = Time::zero();
		Player { race, ready_time }
	}

	pub fn race(&self) -> Race {
		self.race
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.get_terrain(loc);
		let speed = self.race.speed(terrain);
		speed > 0.0
	}

	/// Used for normal movement, i.e. not something like a teleport.
	pub fn on_moved(&mut self, terrain: Terrain, dx: i32, dy: i32) {
		let speed = self.race.speed(terrain);
		let scaling = if dx != 0 && dy != 0 {
			1.414 * speed
		} else {
			speed
		};
		self.ready_time = self.ready_time + (scaling * (Player::MOVEMENT_SPEED as f32)) as i64;
	}
}

impl Scheduled for Player {
	fn ready_time(&self) -> Time {
		self.ready_time
	}

	fn execute(&mut self, level: &mut Level) {
		assert!(false, "execute shouldn't be called on the player");
	}
}

impl MovementSpeed for Race {
	fn speed(&self, terrain: Terrain) -> f32 {
		match self {
			Race::Human => match terrain {
				Terrain::Blank => {
					assert!(false); // blank should only be used for rendering
					0.0
				}
				Terrain::DeepWater => 0.0,
				Terrain::Ground => 1.0,
				Terrain::ShallowWater => 0.9,
				Terrain::Wall => 0.0,
			},
			// Race::Toblakai => match terrain {
			// 	Terrain::Blank => {
			// 		assert!(false); // blank should only be used for rendering
			// 		0.0
			// 	}
			// 	Terrain::DeepWater => 0.0,
			// 	Terrain::Ground => 1.1,
			// 	Terrain::ShallowWater => 1.0,
			// 	Terrain::Wall => 0.0,
			// },
		}
	}
}

impl MovementSpeed for Player {
	fn speed(&self, terrain: Terrain) -> f32 {
		self.race.speed(terrain)
	}
}
