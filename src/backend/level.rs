use std::collections::HashMap; // TODO: may want to use a faster hash
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static ENTITY_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.
#[derive(Clone, Copy, Debug)]
pub struct Entity {
	prefix: &'static str, // static so that we can cheaply copy these
	count: usize,
}

impl Entity {
	pub fn new(prefix: &'static str) -> Entity {
		Entity {
			prefix,
			count: ENTITY_COUNTER.fetch_add(1, Ordering::SeqCst),
		}
	}
}

impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
		self.count == other.count
	}
}

impl Eq for Entity {}

impl Hash for Entity {
	fn hash<S: Hasher>(&self, state: &mut S) {
		self.count.hash(state); // count is the unique part of an Enity so we can save time by ignoring prefix
	}
}

impl slog::Value for Entity {
	fn serialize(
		&self,
		_: &slog::Record<'_>,
		key: slog::Key,
		serializer: &mut dyn slog::Serializer,
	) -> Result<(), slog::Error> {
		serializer.emit_arguments(key, &format_args!("{}", self.count))
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

struct Level {
	player_components: HashMap<Entity, PlayerComponent>,
	position_components: HashMap<Entity, PositionComponent>,
}
