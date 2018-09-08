use super::*;

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Species {
	Ay,    // giant wolf
	Bison, // large herbivore	TODO: is there a better name for this?
}

#[derive(Clone, Deserialize, Serialize)]
pub struct NPC {
	species: Species,
}

impl NPC {
	pub fn new(species: Species) -> NPC {
		NPC { species }
	}

	pub fn species(&self) -> Species {
		self.species
	}

	pub fn can_move_to(&self, level: &Level, loc: Location) -> bool {
		let terrain = level.get_terrain(loc);
		let speed = self.speed(terrain);
		speed > 0.0
	}
}

impl MovementSpeed for Species {
	fn speed(&self, terrain: Terrain) -> f32 {
		match self {
			Species::Ay | Species::Bison => match terrain {
				Terrain::Blank => {
					assert!(false); // blank should only be used for rendering
					0.0
				}
				Terrain::DeepWater => 0.0,
				Terrain::Ground => 1.0,
				Terrain::ShallowWater => 0.9,
				Terrain::Wall => 0.0,
			},
		}
	}
}

impl MovementSpeed for NPC {
	fn speed(&self, terrain: Terrain) -> f32 {
		self.species.speed(terrain)
	}
}
