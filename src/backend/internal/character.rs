use super::*;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Species {
	Ay,       // giant wolf
	Bhederin, // large herbivore
	Human,
	// Toblakai,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum CharacterFlags {
	// Large,
	// Small,

	Airborne,
	Aquatic,
}

pub struct CharacterComponent {
	pub species: Species,
	pub flags: Flags<CharacterFlags>,
}

impl CharacterComponent {
	pub fn new(species: Species, flags: Flags<CharacterFlags>) -> CharacterComponent {
		CharacterComponent { species, flags }
	}
}

impl Species {
	/// Base time it takes for a species to move through a terrain. INFINITE_DURATION if the
	/// terrain is impassable.
	pub fn move_duration(self, terrain: Terrain) -> Duration {
		match terrain {
			Terrain::Blank => panic!("Blank should only be used for rendering"),
			Terrain::DeepWater | Terrain::Wall => INFINITE_DURATION,
			Terrain::Ground => match self {
				Species::Ay | Species::Human => BASE_MOVEMENT_DURATION,
				Species::Bhederin => BASE_MOVEMENT_DURATION.percent(1.2),
			},
			Terrain::ShallowWater => match self {
				Species::Ay | Species::Human => BASE_MOVEMENT_DURATION,
				Species::Bhederin => BASE_MOVEMENT_DURATION.percent(1.2),
			},
		}
	}
}

impl fmt::Display for Species {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{:?}", self)
	}
}
