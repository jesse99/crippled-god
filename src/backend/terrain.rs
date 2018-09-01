use std::fmt;

#[derive(Clone, Copy)]
pub enum Terrain {
	/// This is used for rendering (for a Cell the user hasn't ever seen).
	Blank,
	DeepWater,
	Ground,
	ShallowWater,
	Wall,
}

pub trait MovementSpeed {
	/// This is a multipler applied to the default movement speed, e.g. if 0.8
	/// is returned for ShallowWater then the character speed is 20% slowe than
	/// normal.
	fn speed(&self, terrain: Terrain) -> f32;
}

impl fmt::Debug for Terrain {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Terrain::Blank => write!(f, "?"),
			Terrain::DeepWater => write!(f, "w"),
			Terrain::Ground => write!(f, "."),
			Terrain::ShallowWater => write!(f, "~"),
			Terrain::Wall => write!(f, "#"),
		}
	}
}
