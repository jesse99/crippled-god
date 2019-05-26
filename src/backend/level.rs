
use super::location::Location;
use super::size::Size;
use super::terrain::Terrain;
use super::vec2d::Vec2d;
//use fnv::FnvHashMap;
use std::hash::{Hash, Hasher};
// Usually entities are indexes into a Vec. But:
// 1) An index isn't very meaningful in isolation.
// 2) Speed isn't a huge concern here so the contiguousness of a Vec isn't too important.
// 3) If we did use a Vec we'd wind up with lots of holes as the player kills off monsters.

/// This is a unique identifier for a game object, e.g. the player, a monster, or piece of equipment.
/// Note that these are unique across the whole game, not just the current level.
#[derive(Clone, Copy)]
pub struct Entity {
	prefix: &'static str, // static so that we can cheaply copy these, TODO: not sure that this will work with serialization
	id: usize,
}

impl Entity {
	fn new(prefix: &'static str, id: usize) -> Entity {
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

// struct PlayerComponent {
// 	name: String,
// }

// top-left is (0, 0)
// struct PositionComponent {
// 	x: i32,
// 	y: i32,
// }

/// This contains all the data associated with the current level. Note that when a new level is
/// generated all comnponents with a position are removed except for the player and (some) NPCs
/// near the player.
pub struct Level {
	num_entities: usize, // this is the total number of entities that have ever existed
	// player_components: FnvHashMap<Entity, PlayerComponent>,
	// position_components: FnvHashMap<Entity, PositionComponent>,
	terrain: Vec2d<Terrain>,
}

impl Level {
	/// Creates a new level with no components.
	pub fn new() -> Level {
		// TODO: should this be public?
		let size = Size::new(20, 15);
		//		let size = Size::new(64, 32);
		let mut level = Level {
			num_entities: 0,
			// player_components: FnvHashMap::default(),
			// position_components: FnvHashMap::default(),
			terrain: Vec2d::new(size, Terrain::Ground),
		};

		// Add walls around the outside
		for x in 0..size.width {
			level.set_terrain(x, 0, Terrain::Wall);
			level.set_terrain(x, size.height - 1, Terrain::Wall);
		}
		for y in 0..size.height {
			level.set_terrain(0, y, Terrain::Wall);
			level.set_terrain(size.width - 1, y, Terrain::Wall);
		}

		// Add a little lake in the middle.
		let x = size.width / 2;
		let y = size.height / 2 - 1;
		level.set_terrain(x, y, Terrain::ShallowWater);
		level.set_terrain(x - 1, y + 1, Terrain::DeepWater);
		level.set_terrain(x, y + 1, Terrain::DeepWater);
		level.set_terrain(x + 1, y + 1, Terrain::ShallowWater);
		level.set_terrain(x, y + 2, Terrain::ShallowWater);

		// Add a short wall.
		let y = 8;
		level.set_terrain(x + 2, y, Terrain::Wall);
		level.set_terrain(x + 1, y, Terrain::Wall);
		level.set_terrain(x, y, Terrain::Wall);
		level.set_terrain(x - 1, y, Terrain::Wall);
		level.set_terrain(x - 2, y, Terrain::Wall);

		level
	}

	/// Creates a new enity with no components. The prefix is an arbitrary string literal used
	/// for debugging.
	pub fn new_entity(&mut self, prefix: &'static str) -> Entity {
		// TODO: should this be public?
		self.num_entities += 1;
		Entity::new(prefix, self.num_entities)
	}

	fn set_terrain(&mut self, x: i32, y: i32, terrain: Terrain) {
		let cell = self.terrain.get_mut(Location::new(x, y));
		*cell = terrain;
	}
}
