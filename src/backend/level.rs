use std::collections::HashMap; // TODO: may want to use a faster hash
use std::hash::{Hash, Hasher};

// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.
#[derive(Clone, Copy, Debug)]
pub struct Entity {
	prefix: &'static str, // static so that we can cheaply copy these
	id: usize,
}

impl Entity {
	fn new(prefix: &'static str, id: usize) -> Entity {
		Entity {
			prefix,
			id,
		}
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
		serializer.emit_arguments(key, &format_args!("{}", self.id))
	}
}

struct PlayerComponent {
	name: String,
}

// top-left is (0, 0)
struct PositionComponent {
	x: i32,
	y: i32,
}

pub struct Level {
	player_components: HashMap<Entity, PlayerComponent>,
	position_components: HashMap<Entity, PositionComponent>,
	num_entities: usize,	// this is the total number of entities that have ever existed
}

impl Level {
	pub fn new() -> Level {
		Level {
			player_components: HashMap::new(),
			position_components: HashMap::new(),
			num_entities: 0,
		}
	}

	pub fn new_entity(&mut self, prefix: &'static str) -> Entity {
		self.num_entities += 1;
		Entity::new(prefix, self.num_entities)
	}
}
