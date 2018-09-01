use super::geography::Geography;
use super::level::Level;
use super::location::Location;
use super::terrain::*;
use rand;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Race {
	Human,
	Toblakai,
}

#[derive(Clone)]
pub struct Player {
	pub race: Race,
	pub loc: Location,
}

impl Player {
	pub fn new(race: Race, geography: &Geography, rng: &mut rand::XorShiftRng) -> Player {
		let loc = geography
			.find_loc_with(rng, |t| race.speed(t) > 0.0)
			.expect("failed to find a location when new'ing the player");
		Player { race, loc }
	}

	pub fn is_at(&self, loc: Location) -> bool {
		self.loc == loc
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.geography.at(loc);
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
			Race::Toblakai => match terrain {
				Terrain::Blank => {
					assert!(false); // blank should only be used for rendering
					0.0
				}
				Terrain::DeepWater => 0.0,
				Terrain::Ground => 1.1,
				Terrain::ShallowWater => 1.0,
				Terrain::Wall => 0.0,
			},
		}
	}
}

impl MovementSpeed for Player {
	fn speed(&self, terrain: Terrain) -> f32 {
		self.race.speed(terrain)
	}
}

impl fmt::Debug for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.loc.x, self.loc.y)
	}
}
