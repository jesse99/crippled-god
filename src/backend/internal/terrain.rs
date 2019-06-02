#[derive(Clone, Copy)]
pub enum Terrain {
	/// This is only used for rendering. It's a cell that the user has not ever seen (and may not
	/// actually exist).
	Blank,
	DeepWater,
	Ground,
	ShallowWater,
	Wall,
}

// pub trait MovementDelay {
// 	/// Amount of time it takes a character to move through the terrain. If INFINITY then the
// 	/// character can't move through the terrain.
// 	fn delay(&self, terrain: Terrain) -> f32;
// }

// impl fmt::Debug for Terrain {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match *self {
// 			Terrain::DeepWater => write!(f, "w"),
// 			Terrain::Ground => write!(f, "."),
// 			Terrain::ShallowWater => write!(f, "~"),
// 			Terrain::Wall => write!(f, "#"),
// 		}
// 	}
// }

pub trait BlocksLOS {
	fn blocks_los(&self) -> bool; // TODO: should probably take something like a race
}

impl BlocksLOS for Terrain {
	fn blocks_los(&self) -> bool {
		match *self {
			Terrain::Blank => panic!("Blank should only be used for rendering"),
			Terrain::DeepWater => false,
			Terrain::Ground => false,
			Terrain::ShallowWater => false,
			Terrain::Wall => true,
		}
	}
}

impl slog::Value for Terrain {
	fn serialize(
		&self,
		_: &slog::Record<'_>,
		key: slog::Key,
		serializer: &mut dyn slog::Serializer,
	) -> Result<(), slog::Error> {
		match *self {
			Terrain::Blank => serializer.emit_arguments(key, &format_args!("?")),
			Terrain::DeepWater => serializer.emit_arguments(key, &format_args!("w")),
			Terrain::Ground => serializer.emit_arguments(key, &format_args!(".")),
			Terrain::ShallowWater => serializer.emit_arguments(key, &format_args!("~")),
			Terrain::Wall => serializer.emit_arguments(key, &format_args!("#")),
		}
	}
}
