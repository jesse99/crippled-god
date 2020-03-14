use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terrain {
	DeepWater,
	Ground,
	ShallowWater,
	Wall,
}

impl fmt::Display for Terrain {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
