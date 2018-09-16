use super::npc::*;
use super::pov::*;
use super::rng::*;
use super::scheduled::*;
use super::vec2::*;
use super::*;
use backend::terrain::MovementDelay;
// use rand::SeedableRng;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::f32;
use std::fmt;
use std::mem;

/// Set if the player or an NPC is within a Tile on the Level.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CharacterType {
	Player(Race),
	NPC(Species),
	None,
}

/// Used to render a location within the Level.
#[derive(Clone, Deserialize, Serialize)]
pub struct Tile {
	/// If visible is set then terrain and character will be up to date. Otherwise terrain will be
	/// blank if the user has never seen the Tile and whatever he saw last if he has seen the Tile.
	pub terrain: Terrain,
	pub character: CharacterType,

	/// True if the player can see the square
	pub visible: bool,
}

/// Contains all the info for a level in the game.
#[derive(Deserialize, Serialize)]
pub struct Level {
	cells: Vec2<Cell>,
	tiles: Vec2<Tile>,
	player_loc: Location,
	npc_locs: FnvHashSet<Location>,
}

impl Level {
	pub fn new(rng: &mut RNG) -> Level {
		let size = Size::new(64, 32);
		let cells = Vec2::new(size, Level::DEFAULT_CELL);
		let tiles = Vec2::new(size, Level::DEFAULT_TILE);
		let npc_locs = FnvHashSet::default();
		let mut level = Level {
			cells,
			tiles,
			player_loc: Location::new(0, 0),
			npc_locs,
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
		level.set_terrain(x + 0, y, Terrain::Wall);
		level.set_terrain(x - 1, y, Terrain::Wall);
		level.set_terrain(x - 2, y, Terrain::Wall);

		// Add the player.
		let race = Race::Human;
		let player = Player::new(race);
		let loc = level
			.rand_loc_for_char(rng, |t| race.delay(t) < f32::INFINITY)
			.expect("failed to find a location when new'ing the player");
		level.add_player(loc, player);

		// Add some NPCs.
		for _ in 0..5 {
			let species = Species::Ay;
			let npc = NPC::new(species);
			let loc = level
				.rand_loc_for_char(rng, |t| species.delay(t) < f32::INFINITY)
				.expect("failed to find a location when new'ing an Ay");
			level.add_npc(loc, npc);
		}

		for _ in 0..5 {
			let species = Species::Bhederin;
			let npc = NPC::new(species);
			let loc = level
				.rand_loc_for_char(rng, |t| species.delay(t) < f32::INFINITY)
				.expect("failed to find a location when new'ing a Bison");
			level.add_npc(loc, npc);
		}

		level
	}

	pub fn player(&self) -> &Player {
		let cell = self.cells.get(self.player_loc);
		match cell.character {
			Character::Player(ref p) => p,
			_ => {
				assert!(false, "{:?} does not contain a player", cell);
				panic!()
			}
		}
	}

	pub fn player_mut(&mut self) -> &mut Player {
		let cell = self.cells.get_mut(self.player_loc);
		match cell.character {
			Character::Player(ref mut p) => p,
			_ => {
				assert!(false, "{:?} does not contain a player", cell);
				panic!()
			}
		}
	}

	pub fn player_loc(&self) -> Location {
		self.player_loc
	}

	pub fn move_player(&mut self, new_loc: Location) {
		assert!(self.empty(new_loc));
		let old_loc = self.player_loc;
		let mut tmp = Character::None;
		{
			let old_cell = self.cells.get_mut(self.player_loc);
			mem::swap(&mut old_cell.character, &mut tmp);
		}

		let terrain = {
			let new_cell = self.cells.get_mut(new_loc);
			mem::swap(&mut tmp, &mut new_cell.character);
			new_cell.terrain
		};
		self.player_loc = new_loc;
		self.player_mut()
			.on_moved(terrain, new_loc.x - old_loc.x, new_loc.y - old_loc.y);
	}

	pub fn empty(&self, loc: Location) -> bool {
		let cell = self.cells.get(loc);
		match cell.character {
			Character::NPC(_) => false,
			Character::Player(_) => false,
			Character::None => true,
		}
	}

	pub fn npc(&self, loc: Location) -> &NPC {
		let cell = self.cells.get(loc);
		match cell.character {
			Character::NPC(ref c) => c,
			_ => {
				assert!(false, "{:?} does not contain an npc", cell);
				panic!()
			}
		}
	}

	// pub fn npc_mut(&mut self, loc: Location) -> &mut NPC {
	// 	let cell = self.cells.get_mut(loc);
	// 	match cell.character {
	// 		Character::NPC(ref mut c) => c,
	// 		_ => {
	// 			assert!(false, "{:?} does not contain an npc", cell);
	// 			panic!()
	// 		}
	// 	}
	// }

	// pub fn npc_moved(&mut self, old_loc: Location, new_loc: Location) {
	// 	assert!(self.empty(new_loc));
	// 	let mut tmp = Character::None;
	// 	{
	// 		let old_cell = self.cells.get_mut(old_loc);
	// 		mem::swap(&mut old_cell.character, &mut tmp);
	// 	}

	// 	let terrain = {
	// 		let new_cell = self.cells.get_mut(new_loc);
	// 		mem::swap(&mut tmp, &mut new_cell.character);
	// 		new_cell.terrain
	// 	};

	// 	self.npc_locs.remove(&old_loc);
	// 	self.npc_locs.insert(new_loc);
	// 	self.player_mut()
	// 		.on_moved(terrain, new_loc.x - old_loc.x, new_loc.y - old_loc.y);
	// }

	/// Returns the next time at which a monster or device is ready to execute, i.e. everything but
	/// the player.
	pub fn other_ready_time(&self) -> Option<Time> {
		let times = self.npc_locs.iter().map(|loc| self.npc(*loc).ready_time());
		let time = times.min();
		// info!("next NPC will be ready at {:?}", time);
		time
	}

	// Normally scheduling would happen with a priority queue but that would require something like
	// Rc which gets annoying. So we simply store one reference and brute force scheduling which
	// should be fine given our relatively small levels.
	pub fn execute_others(&mut self, game_time: Time, rng: &mut RNG) {
		let locs: Vec<Location> = self
			.npc_locs
			.iter()
			.filter(|loc| {
				let npc = self.npc(**loc);
				assert!(npc.ready_time() >= game_time);
				npc.ready_time() == game_time
			})
			.map(|loc| *loc)	// need the map and the collect to appease the borrow checker
			.collect();
		for loc in locs {
			let mut npc = self.remove_npc(loc);
			if let Some(new_loc) = npc.execute(self, loc, rng) {
				self.add_npc(new_loc, npc);
			}
		}
	}

	pub fn get_terrain(&self, loc: Location) -> Terrain {
		self.cells.get(loc).terrain
	}

	/// Returns a randomized location that satisfies the predicate.
	pub fn rand_loc_for_char<T>(&self, rng: &mut RNG, predicate: T) -> Option<Location>
	where
		T: Fn(Terrain) -> bool,
	{
		let size = self.cells.size();
		let mut indexes: Vec<i32> = (0..size.width * size.height).collect();
		rng.shuffle(&mut indexes);

		for i in indexes.iter() {
			let x = i % size.width;
			let y = i / size.width;
			let loc = Location::new(x, y);
			let cell = self.cells.get(loc);
			if let Character::None = cell.character {
				let terrain = cell.terrain;
				if predicate(terrain) {
					return Some(loc);
				}
			}
		}
		return None;
	}

	/// screen_size is the number of tiles the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho that info may be outdated). It can also be arbitrarily small
	/// though in that case the user may not be able to see all the tiles the player can.
	/// Note that this is normally accessed through the Game method with the same name.
	pub fn get_tiles(&mut self, screen_size: Size) -> Vec2<Tile> {
		self.update_tiles();
		self.screen_tiles(screen_size)
	}

	// ---- Private Items ---------------------------------------------------------------
	fn set_terrain(&mut self, x: i32, y: i32, terrain: Terrain) {
		let cell = self.cells.get_mut(Location::new(x, y));
		cell.terrain = terrain;
	}

	fn add_player(&mut self, loc: Location, player: Player) {
		assert!(self.empty(loc));
		let cell = self.cells.get_mut(loc);
		cell.character = Character::Player(player);
		self.player_loc = loc;
	}

	fn add_npc(&mut self, loc: Location, npc: NPC) {
		assert!(self.empty(loc));
		let cell = self.cells.get_mut(loc);
		cell.character = Character::NPC(npc);
		self.npc_locs.insert(loc);
	}

	fn remove_npc(&mut self, loc: Location) -> NPC {
		assert!(!self.empty(loc));
		let mut tmp = Character::None;
		let old_cell = self.cells.get_mut(loc);
		mem::swap(&mut old_cell.character, &mut tmp);
		self.npc_locs.remove(&loc);
		match tmp {
			Character::NPC(c) => c,
			_ => {
				assert!(false, "{:?} doesn't contain an npc", tmp);
				panic!()
			}
		}
	}

	// Returns the subset of tiles that are rendered on the screen.
	fn screen_tiles(&self, screen_size: Size) -> Vec2<Tile> {
		let mut tiles = Vec2::new(screen_size, Level::DEFAULT_TILE);
		let start_x = self.player_loc.x - screen_size.width / 2;
		let start_y = self.player_loc.y - screen_size.height / 2;
		for out_y in 0..screen_size.height {
			for out_x in 0..screen_size.width {
				let in_loc = Location::new(start_x + out_x, start_y + out_y);
				if in_loc.x >= 0
					&& in_loc.x < self.tiles.size().width
					&& in_loc.y >= 0
					&& in_loc.y < self.tiles.size().height
				{
					let tile = self.tiles.get(in_loc);

					let out_loc = Location::new(out_x, out_y);
					tiles.set(out_loc, tile.clone());
				}
			}
		}
		tiles
	}

	const DEFAULT_CELL: Cell = Cell {
		terrain: Terrain::Ground,
		character: Character::None,
	};

	const DEFAULT_TILE: Tile = Tile {
		terrain: Terrain::Blank,
		character: CharacterType::None,
		visible: false,
	};

	// Updates the tiles that are within the player's LOS.
	fn update_tiles(&mut self) {
		// The borrow checker won't allow us to grab a mutable reference to tiles in one closure and
		// another reference in the second closure so we need to figure out what we need to do before
		// we call apply.
		let mut visible = FnvHashMap::default();
		{
			let visit = |loc| {
				let cell = self.cells.get(loc);
				match cell.character {
					Character::Player(ref p) => {
						visible.insert(loc, (cell.terrain, CharacterType::Player(p.race())))
					}
					Character::NPC(ref c) => {
						visible.insert(loc, (cell.terrain, CharacterType::NPC(c.species())))
					}
					Character::None => visible.insert(loc, (cell.terrain, CharacterType::None)),
				};
			};
			let blocks = |loc| {
				let terrain = self.get_terrain(loc);
				terrain.blocks_los()
			};
			let radius = 10; // TODO: depends on race?
			visit_visible_tiles(self.player_loc, self.tiles.size(), radius, visit, blocks);
		}

		self.tiles.apply(|loc, tile| match visible.get(&loc) {
			Some((terrain, ch)) => {
				tile.terrain = *terrain;
				tile.character = *ch;
				tile.visible = true;
			}
			None => tile.visible = false,
		})
	}

	/// Returns true if loc is visible from start_loc,
	pub fn is_visible(&self, start_loc: Location, loc: Location) -> bool {
		let mut visible = false;
		{
			let visit = |l| {
				if l == loc {
					visible = true;
				}
			};
			let blocks = |l| {
				let terrain = self.get_terrain(l);
				terrain.blocks_los()
			};
			let radius = 10; // TODO: depends on race?
			visit_visible_tiles(start_loc, self.tiles.size(), radius, visit, blocks);
		}

		visible
	}
}

/// Set if the player or an NPC is within a Tile on the Level.
#[derive(Clone, Debug, Deserialize, Serialize)]
enum Character {
	Player(Player),
	NPC(NPC),
	None,
}

/// Level has a 2D array of these.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Cell {
	terrain: Terrain,
	character: Character,
	// feature: Option<Feature>,
	// items: Vec<Item>,
}

impl fmt::Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.character {
			CharacterType::Player(_) => write!(f, "@"),
			CharacterType::NPC(_) => write!(f, "m"),
			CharacterType::None => write!(f, "{:?}", self.terrain),
		}
	}
}

impl fmt::Debug for Level {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n")?;
		for y in 0..self.cells.size().height {
			for x in 0..self.cells.size().width {
				let loc = Location::new(x, y);
				write!(f, "{:?}", self.get_terrain(loc))?;
			}
			if y + 1 < self.cells.size().height {
				write!(f, "\n")?;
			}
		}
		write!(f, "")
	}
}
