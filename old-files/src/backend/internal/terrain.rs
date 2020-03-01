
use super::super::Game;
use super::*;
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Terrain {
	/// This is only used for rendering. It's a cell that the user has not ever seen (and may not
	/// actually exist).
	DeepWater,
	Ground,
	ShallowWater,
	Wall,
}

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
			Terrain::DeepWater => false,
			Terrain::Ground => false,
			Terrain::ShallowWater => false,
			Terrain::Wall => true,
		}
	}
}

impl MessageFor for Terrain {
	fn message_for(&self, game: &Game, entity: Entity) -> Option<Message> {
		if game.is_player(entity) {
			match *self {
				Terrain::DeepWater => Some(Message {
					topic: Topic::NonGamePlay,
					text: "That water is too deep.".to_string(),
				}),
				Terrain::Ground => None,
				Terrain::ShallowWater => Some(Message {
					topic: Topic::PlayerIsImpaired,
					text: "You splash into the water.".to_string(),
				}), // TODO: this should have some impairment, probably noise and slower movement
				Terrain::Wall => None,
			}
		} else {
			None
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
			Terrain::DeepWater => serializer.emit_arguments(key, &format_args!("w")),
			Terrain::Ground => serializer.emit_arguments(key, &format_args!(".")),
			Terrain::ShallowWater => serializer.emit_arguments(key, &format_args!("~")),
			Terrain::Wall => serializer.emit_arguments(key, &format_args!("#")),
		}
	}
}
