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
	pub position_components: FnvHashMap<Entity, Location>, // TODO: do we need a map for the opposite direction?
	pub cells: Vec2d<Cell>,
	pub logger: Logger,
	pub rng: RNG,

	num_entities: usize, // this is the total number of entities that have ever existed
}

// TODO: add an invariant for debug builds
impl Level {
	/// Creates a new level with just a player component.
	pub fn with_logger(game_logger: &Logger, rng: RNG) -> Level {
		// TODO: should this be public?
		let level_logger = game_logger.new(o!());

		let size = Size::new(64, 32);
		let player = Entity::internal_new("player", 1);
		let default_cell = Cell {
			terrain: Terrain::Ground,
			character: None,
		};
		let mut level = Level {
			player,
			num_entities: 1,
			character_components: FnvHashMap::default(),
			position_components: FnvHashMap::default(),
			cells: Vec2d::new(size, default_cell),
			logger: level_logger,
			rng,
		};

		let flags = Flags::<CharacterFlags>::new();
		level
			.character_components
			.insert(player, CharacterComponent::new(Species::Human, flags));
		let player_loc = Location::new(1, 1);
		level.position_components.insert(player, player_loc);
		level.cells.get_mut(player_loc).character = Some(player);

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

		// Add some NPCs.
		for _ in 0..5 {
			let species = Species::Ay;
			let flags = Flags::<CharacterFlags>::new();
			let npc = CharacterComponent::new(species, flags);
			let loc = level
				.rand_loc_for_char(|cell| species.move_duration(cell.terrain) < INFINITE_DURATION)
				.expect("failed to find a location when new'ing an Ay");
			level.add_npc(loc, npc, "Ay");
		}

		for _ in 0..5 {
			let species = Species::Bhederin;
			let flags = Flags::<CharacterFlags>::new();
			let npc = CharacterComponent::new(species, flags);
			let loc = level
				.rand_loc_for_char(|cell| species.move_duration(cell.terrain) < INFINITE_DURATION)
				.expect("failed to find a location when new'ing a Bison");
			level.add_npc(loc, npc, "Bhederin");
		}

		level
	}

	/// Returns a randomized location that satisfies the predicate.
	pub fn rand_loc_for_char<T>(&mut self, predicate: T) -> Option<Location>
	where
		T: Fn(&Cell) -> bool,
	{
		let size = self.cells.size();
		let mut indexes: Vec<i32> = (0..size.width * size.height).collect();
		self.rng.shuffle(&mut indexes);

		for i in &indexes {
			let x = i % size.width;
			let y = i / size.width;
			let loc = Location::new(x, y);
			let cell = self.cells.get(loc);
			if let None = cell.character {
				if predicate(cell) {
					return Some(loc);
				}
			}
		}
		None
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

	fn add_npc(&mut self, loc: Location, npc: CharacterComponent, name: &'static str) {
		let entity = self.new_entity(name);
		let flags = Flags::<CharacterFlags>::new();
		self.character_components.insert(entity, npc);
		self.position_components.insert(entity, loc);
		self.cells.get_mut(loc).character = Some(entity);
	}
}
