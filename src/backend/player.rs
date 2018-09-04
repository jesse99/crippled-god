use super::*;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Race {
	Human,
	// Toblakai,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Player {
	race: Race,
}

impl Player {
	pub fn new(race: Race) -> Player {
		Player { race }
	}

	pub fn race(&self) -> Race {
		self.race
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.geography().at(loc);
		let speed = self.race.speed(terrain);
		speed > 0.0
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
