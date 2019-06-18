use super::*;

use fnv::FnvHashMap;
use slog::Logger;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct Cell {
	pub terrain: Terrain,
	pub character: Option<Entity>,
	// pub objects: Vec<Entity>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scheduled {
	pub entity: Entity,
	pub time: Time,
}

impl Ord for Scheduled {
	fn cmp(&self, other: &Scheduled) -> Ordering {
		other.time.cmp(&self.time) // backwards so our priority heap returns min times first
	}
}

impl PartialOrd for Scheduled {
	fn partial_cmp(&self, other: &Scheduled) -> Option<Ordering> {
		Some(self.cmp(other))
	}
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
	pub scheduled: BinaryHeap<Scheduled>,

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
			scheduled: BinaryHeap::new(),
		};

		let flags = Flags::<CharacterFlags>::new();
		level
			.character_components
			.insert(player, CharacterComponent::new(Species::Human, flags));
		let player_loc = Location::new(1, 1);
		level.position_components.insert(player, player_loc);
		level.scheduled.push(Scheduled {
			entity: player,
			time: Time::from_seconds(0),
		});
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

		level.invariant();

		level
	}

	pub fn player_loc(&self) -> Location {
		*(self
			.position_components
			.get(&self.player)
			.expect("Player has no position"))
	}

	pub fn entity_loc(&self, entity: Entity) -> Option<Location> {
		self.position_components.get(&entity).map(|r| *r)
	}

	/// Returns true if loc is visible from start_loc.
	pub fn is_visible(&self, start_loc: Location, loc: Location) -> bool {
		let mut visible = false;
		{
			let mut pov = pov::POV {
				start: start_loc,
				size: self.cells.size(),
				radius: 10, // TODO: depends on race?
				visit_tile: |l| {
					if l == loc {
						visible = true;
					}
				},
				blocks_los: |l| {
					let cell = self.cells.get(l);
					cell.terrain.blocks_los()
				},
			};

			pov.visit();
		}

		visible
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
			if cell.character.is_none() && predicate(cell) {
				return Some(loc);
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
		self.character_components.insert(entity, npc);
		self.position_components.insert(entity, loc);
		self.cells.get_mut(loc).character = Some(entity);

		let c = *(self.scheduled.peek().unwrap());
		self.scheduled.push(Scheduled {
			entity,
			time: c.time + Duration::from_seconds(1),
		});
		self.invariant();
	}

	pub fn remove_entity(&mut self, entity: Entity) {
		if let Some(loc) = self.position_components.get(&entity) {
			let cell = self.cells.get_mut(*loc);
			cell.character = None;
		}

		self.character_components.remove(&entity);
		self.position_components.remove(&entity);
		self.invariant();
	}

	pub fn invariant(&self) {
		// player should be in character_components
		assert!(self.character_components.get(&self.player).is_some());

		// all characters should have a position and be scheduled (this is a bit of a weak test
		// which is why we have a more rigorous test below).
		assert!(self.character_components.len() == self.position_components.len());
		assert!(self.character_components.len() == self.scheduled.len());

		// position entities must exactly match character entities
		let mut char_entities: Vec<Entity> = self.character_components.keys().cloned().collect();
		char_entities.sort();
		let mut pos_entities: Vec<Entity> = self.position_components.keys().cloned().collect();
		pos_entities.sort();
		assert!(char_entities == pos_entities);

		// scheduled entities must exactly match character entities
		let mut sched_entities: Vec<Entity> = self.scheduled.iter().map(|s| s.entity).collect();
		sched_entities.sort();
		assert!(pos_entities == sched_entities);

		// every entity with a position should be within cells
		let mut cell_entities: Vec<Entity> =
			self.cells.iter().filter_map(|c| c.1.character).collect();
		cell_entities.sort();
		assert!(pos_entities == cell_entities);

		// scheduled times are sane
		for s in &self.scheduled {
			assert!(s.time.0 >= 0);
			assert!(s.time.0 < 1000);
		}
	}
}
