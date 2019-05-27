
use super::character::{CharacterComponent, CharacterFlags};
use super::entity::Entity;
use super::flags::Flags;
use super::location::Location;
use super::size::Size;
use super::terrain::Terrain;
use super::vec2d::Vec2d;
use fnv::FnvHashMap;

/// This contains all the data associated with the current level. Note that when a new level is
/// generated all comnponents with a position are removed except for the player and (some) NPCs
/// near the player.
pub struct Level {
	pub player: Entity,
	pub character_components: FnvHashMap<Entity, CharacterComponent>,
	pub position_components: FnvHashMap<Entity, Location>,

	num_entities: usize, // this is the total number of entities that have ever existed
	terrain: Vec2d<Terrain>,
}

impl Level {
	/// Creates a new level with just a player component.
	pub fn new() -> Level {
		// TODO: should this be public?
		let size = Size::new(64, 32);
		let mut level = Level {
			player: Entity::internal_new("player", 1),
			num_entities: 1,
			character_components: FnvHashMap::default(),
			position_components: FnvHashMap::default(),
			terrain: Vec2d::new(size, Terrain::Ground),
		};

		let player = level.new_entity("player");
		let flags = Flags::<CharacterFlags>::new();
		level.character_components.insert(
			player,
			CharacterComponent::new("player", flags)
		);
		level.position_components.insert(
			player,
			Location::new(1, 1),
		);

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
		Entity::internal_new(prefix, self.num_entities)
	}

	fn set_terrain(&mut self, x: i32, y: i32, terrain: Terrain) {
		let cell = self.terrain.get_mut(Location::new(x, y));
		*cell = terrain;
	}
}
