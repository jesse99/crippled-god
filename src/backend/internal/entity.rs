// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.
use super::*;
// use fnv::FnvHashMap;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Kind {
	Player,
	NPC(Species),
}

/// This is a unique identifier for a game object, e.g. the player, a monster, or piece of equipment.
/// Note that these are unique across the whole game, not just the current level. Also note that
/// these are created via Level::new_entity.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Entity {
	kind: Kind, 		// could also use a String but we want something cheap to copy here
	id: usize,
}

impl Entity {
	/// Use Level::new_entity instead of this.
	pub fn internal_new(kind: Kind, id: usize) -> Entity {
		Entity { kind, id }
	}
}

impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Entity {}

impl Ord for Entity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for Entity {
	fn hash<S: Hasher>(&self, state: &mut S) {
		self.id.hash(state); // id is the unique part of an Enity so we can save time by ignoring prefix
	}
}

impl slog::Value for Entity {
	fn serialize(
		&self,
		_: &slog::Record<'_>,
		key: slog::Key,
		serializer: &mut dyn slog::Serializer,
	) -> Result<(), slog::Error> {
		serializer.emit_arguments(key, &format_args!("{:?}-{}", self.kind, self.id))
	}
}
