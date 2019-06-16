// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.
// use super::*;
// use fnv::FnvHashMap;
use std::hash::{Hash, Hasher};

/// This is a unique identifier for a game object, e.g. the player, a monster, or piece of equipment.
/// Note that these are unique across the whole game, not just the current level. Also note that
/// these are created via Level::new_entity.
#[derive(Clone, Copy, Debug)]
pub struct Entity {
	prefix: &'static str, // static so that we can cheaply copy these, TODO: not sure that this will work with serialization
	id: usize,
}

impl Entity {
	/// Use Level::new_entity instead of this.
	pub fn internal_new(prefix: &'static str, id: usize) -> Entity {
		Entity { prefix, id }
	}
}

impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Entity {}

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
		serializer.emit_arguments(key, &format_args!("{}-{}", self.prefix, self.id))
	}
}
