use super::character::*;
use super::rng::*;
use super::scheduled::*;
use super::vec2::*;
use super::*;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use std::f32;
use std::fmt;

/// Used to render a location within the Level.
#[derive(Clone, Deserialize, Serialize)]
pub struct Tile {
	/// If visible is set then terrain and character will be up to date. Otherwise terrain will be
	/// blank if the user has never seen the Tile and whatever he saw last if he has seen the Tile.
	pub terrain: Terrain,
	pub char_name: Option<CharName>,
	pub has_player: bool,

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
		level.set_terrain(x, y, Terrain::Wall);
		level.set_terrain(x - 1, y, Terrain::Wall);
		level.set_terrain(x - 2, y, Terrain::Wall);

		// Add the player.
		let name = CharName::Human;
		let player = Character::new_player(name);
		let loc = level
			.rand_loc_for_char(rng, |t| {
				(attributes(name).movement_delay)(t) < f32::INFINITY
			}).expect("failed to find a location when new'ing the player");
		level.add_player(loc, player);

		// Add some NPCs.
		for _ in 0..5 {
			let name = CharName::Ay;
			let npc = Character::new_npc(name);
			let loc = level
				.rand_loc_for_char(rng, |t| {
					(attributes(name).movement_delay)(t) < f32::INFINITY
				}).expect("failed to find a location when new'ing an Ay");
			level.add_npc(loc, npc);
		}

		for _ in 0..5 {
			let name = CharName::Bhederin;
			let npc = Character::new_npc(name);
			let loc = level
				.rand_loc_for_char(rng, |t| {
					(attributes(name).movement_delay)(t) < f32::INFINITY
				}).expect("failed to find a location when new'ing a Bison");
			level.add_npc(loc, npc);
		}

		level.invariant();
		level
	}

	pub fn player(&self) -> &Character {
		let cell = self.cells.get(self.player_loc);
		cell.character.as_ref().unwrap()
	}

	// pub fn player_mut(&mut self) -> &mut Character {
	// 	let cell = self.cells.get_mut(self.player_loc);
	// 	cell.character.as_mut().unwrap()
	// }

	pub fn player_loc(&self) -> Location {
		self.player_loc
	}

	pub fn move_player(&mut self, new_loc: Location) {
		assert!(self.has_char(new_loc));
		self.invariant();

		{
			let old_loc = self.player_loc;
			let player = {
				let old_cell = self.cells.get_mut(self.player_loc);
				old_cell.character.take()
			};
			assert!(player.is_some());

			let new_cell = self.cells.get_mut(new_loc);
			assert!(new_cell.character.is_none());
			new_cell.character = player;

			self.player_loc = new_loc;
			new_cell.character.as_mut().unwrap().on_moved(
				new_cell.terrain,
				new_loc.x - old_loc.x,
				new_loc.y - old_loc.y,
			);
		}

		// let old_loc = self.player_loc;
		// let mut tmp = Character::None;
		// {
		// 	let old_cell = self.cells.get_mut(self.player_loc);
		// 	mem::swap(&mut old_cell.character, &mut tmp);
		// }

		// let terrain = {
		// 	let new_cell = self.cells.get_mut(new_loc);
		// 	mem::swap(&mut tmp, &mut new_cell.character);
		// 	new_cell.terrain
		// };
		// self.player_loc = new_loc;
		// self.player_mut()
		// 	.on_moved(terrain, new_loc.x - old_loc.x, new_loc.y - old_loc.y);
		self.invariant();
	}

	pub fn has_char(&self, loc: Location) -> bool {
		let cell = self.cells.get(loc);
		cell.character.is_none()
	}

	pub fn npc(&self, loc: Location) -> &Character {
		assert!(self.player_loc != loc);
		let cell = self.cells.get(loc);
		cell.character.as_ref().unwrap()
	}

	// pub fn npc_mut(&mut self, loc: Location) -> &mut Character {
	// 	assert!(self.player_loc != loc);
	// 	let cell = self.cells.get_mut(loc);
	// 	cell.character.as_mut().unwrap()
	// }

	// pub fn npc_mut(&mut self, loc: Location) -> &mut Character {
	// 	let cell = self.cells.get_mut(loc);
	// &mut cell.character.unwrap()
	// }

	/// Returns the next time at which a monster or device is ready to execute, i.e. everything but
	/// the player.
	pub fn other_ready_time(&self) -> Option<Time> {
		let times = self.npc_locs.iter().map(|loc| self.npc(*loc).ready_time());
		times.min()
	}

	// Normally scheduling would happen with a priority queue but that would require something like
	// Rc which gets annoying. So we simply store one reference and brute force scheduling which
	// should be fine given our relatively small levels.
	pub fn execute_others(&mut self, game_time: Time, rng: &mut RNG) {
		self.invariant();
		// TODO: We need to make a copy of the locations we need to iterate before we start
		// mutating the vector. It would be more efficient to do this after the filter but the
		// only way I could figure out for that was to use a map to dereference and then call
		// collect but clippy complained that the map was better off replaced with a clone and
		// I coulnd't get the references working that way.
		let old_locs = self.npc_locs.clone();
		let npc_locs: Vec<&Location> = old_locs
			.iter()
			.filter(|loc| {
				let npc = self.npc(**loc);
				assert!(npc.ready_time() >= game_time);
				npc.ready_time() == game_time
			}).collect();
		for &loc in npc_locs {
			let mut npc = self.remove_npc(loc);
			if let Some(new_loc) = npc.execute(self, loc, rng) {
				self.add_npc(new_loc, npc);
			}
		}
		self.invariant();
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

		for i in &indexes {
			let x = i % size.width;
			let y = i / size.width;
			let loc = Location::new(x, y);
			let cell = self.cells.get(loc);
			if let None = cell.character {
				let terrain = cell.terrain;
				if predicate(terrain) {
					return Some(loc);
				}
			}
		}
		None
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

	fn add_player(&mut self, loc: Location, player: Character) {
		assert!(self.has_char(loc));
		{
			let cell = self.cells.get_mut(loc);
			cell.character = Some(player);
			self.player_loc = loc;
		}
		assert!(!self.has_char(loc));
	}

	fn add_npc(&mut self, loc: Location, npc: Character) {
		assert!(self.has_char(loc));
		assert!(self.player_loc != loc);
		{
			let cell = self.cells.get_mut(loc);
			cell.character = Some(npc);
			self.npc_locs.insert(loc);
		}
		assert!(!self.has_char(loc));
	}

	fn remove_npc(&mut self, loc: Location) -> Character {
		assert!(!self.has_char(loc));
		assert!(self.player_loc != loc);

		let character = {
			let cell = self.cells.get_mut(loc);
			let chr = cell.character.take();
			chr.unwrap()
		};
		self.npc_locs.remove(&loc);
		assert!(self.has_char(loc));
		character
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
		character: None,
	};

	const DEFAULT_TILE: Tile = Tile {
		terrain: Terrain::Blank,
		char_name: None,
		has_player: false,
		visible: false,
	};

	// Updates the tiles that are within the player's LOS.
	fn update_tiles(&mut self) {
		// The borrow checker won't allow us to grab a mutable reference to tiles in one closure and
		// another reference in the second closure so we need to figure out what we need to do before
		// we call apply.
		let mut visible = FnvHashMap::default();
		{
			let mut pov = pov::POV {
				start: self.player_loc,
				size: self.tiles.size(),
				radius: 10, // TODO: depends on race?
				visit_tile: |loc| {
					let cell = self.cells.get(loc);
					visible.insert(
						loc,
						(cell.terrain, cell.character.as_ref().map(|c| c.name())),
					);
				},
				blocks_los: |loc| {
					let terrain = self.get_terrain(loc);
					terrain.blocks_los()
				},
			};

			pov.visit();
		}

		let player_loc = self.player_loc;
		self.tiles.apply(|loc, tile| match visible.get(&loc) {
			Some((terrain, ch)) => {
				tile.terrain = *terrain;
				tile.char_name = *ch;
				tile.has_player = loc == player_loc;
				tile.visible = true;
			}
			None => tile.visible = false,
		})
	}

	/// Returns true if loc is visible from start_loc,
	pub fn is_visible(&self, start_loc: Location, loc: Location) -> bool {
		let mut visible = false;
		{
			let mut pov = pov::POV {
				start: start_loc,
				size: self.tiles.size(),
				radius: 10, // TODO: depends on race?
				visit_tile: |l| {
					if l == loc {
						visible = true;
					}
				},
				blocks_los: |l| {
					let terrain = self.get_terrain(l);
					terrain.blocks_los()
				},
			};

			pov.visit();
		}

		visible
	}

	fn invariant(&self) {
		assert!(self.tiles.size() == self.cells.size());

		assert!(self.player_loc.x >= 0);
		assert!(self.player_loc.x < self.cells.size().width);
		assert!(self.player_loc.y >= 0);
		assert!(self.player_loc.y < self.cells.size().height);

		for loc in &self.npc_locs {
			assert!(*loc != self.player_loc);

			assert!(loc.x >= 0);
			assert!(loc.x < self.cells.size().width);
			assert!(loc.y >= 0);
			assert!(loc.y < self.cells.size().height);

			assert!(!self.has_char(*loc));
		}
	}
}

/// Level has a 2D array of these.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Cell {
	terrain: Terrain,
	character: Option<Character>,
	// feature: Option<Feature>,
	// items: Vec<Item>,
}

impl fmt::Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(_) = self.char_name {
			if self.has_player {
				write!(f, "@")
			} else {
				write!(f, "m")
			}
		} else {
			write!(f, "{:?}", self.terrain)
		}
	}
}

impl fmt::Debug for Level {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f)?;
		for y in 0..self.cells.size().height {
			for x in 0..self.cells.size().width {
				let loc = Location::new(x, y);
				write!(f, "{:?}", self.get_terrain(loc))?;
			}
			if y + 1 < self.cells.size().height {
				writeln!(f)?;
			}
		}
		write!(f, "")
	}
}
