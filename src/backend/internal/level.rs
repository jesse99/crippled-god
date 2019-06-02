use super::*;
use fnv::FnvHashMap;
use slog::Logger;

#[derive(Clone)]
pub struct Cell {
	pub terrain: Terrain,
	pub character: Option<Entity>,
	// pub objects: Vec<Entity>,		
}

/// This contains all the data associated with the current level. Note that when a new level is
/// generated all comnponents with a position are removed except for the player and (some) NPCs
/// near the player.
pub struct Level {
	pub player: super::Entity,
	pub character_components: FnvHashMap<Entity, CharacterComponent>,
	pub position_components: FnvHashMap<Entity, Location>,		// TODO: do we need a map for the opposite direction?
	pub cells: Vec2d<Cell>,
	pub logger: Logger,

	num_entities: usize, // this is the total number of entities that have ever existed
}

// TODO: add an invariant for debug builds
impl Level {
	/// Creates a new level with just a player component.
	pub fn with_logger(logger: Logger) -> Level {
		// TODO: should this be public?
		let size = Size::new(64, 32);
		let player = Entity::internal_new("player", 1);
		let default_cell = Cell{terrain: Terrain::Ground, character: None};
		let mut level = Level {
			player,
			num_entities: 1,
			character_components: FnvHashMap::default(),
			position_components: FnvHashMap::default(),
			cells: Vec2d::new(size, default_cell),
			logger,
		};

		let flags = Flags::<CharacterFlags>::new();
		level
			.character_components
			.insert(player, CharacterComponent::new("player", flags));
		level
			.position_components
			.insert(player, Location::new(1, 1));

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
		let cell = self.cells.get_mut(Location::new(x, y));
		cell.terrain = terrain;
	}
}
