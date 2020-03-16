use super::core::*;
use super::player::*;
use fnv::FnvHashMap;

/// Visual state of the player or an NPC.
#[derive(Clone)]
pub struct CharTile {
	// TODO:
	// species
	// weapon id
	// armor id
	// current effects bitfield
	// maybe friendly or hostile
	// Note that we don't want to just use an id for the Character because we need
	// to record the current state of the character in case it moves out of the
	// player's field of view.
	/// UIs will normally want to highlight the player in some way.
	pub is_player: bool,
}

/// Visual state of a cell within the level.
#[derive(Clone)]
pub struct Tile {
	/// If true then the tile is currently within the player's field of view.
	/// If false then the tile state is as it was when the player last saw it.
	pub visible: bool,

	/// Player or NPC.
	pub character: Option<CharTile>,

	/// None if the player has never seen the tile.
	pub terrain: Option<Terrain>,
}

/// Record of the terrain and positions of NPCs and items within a particular dungeon level.
pub struct Level {
	name: String,
	terrain: Vec2d<Terrain>,
	tiles: Vec2d<Tile>,
}

// TODO:
// may want to store NPCs and items in their own services
// if we do that we may want to rename this Map
impl Level {
	/// Levels start out empty and become populated as events occur.
	pub fn new() -> Level {
		Level {
			name: "uninitialized".to_string(),
			terrain: Vec2d::empty(),
			tiles: Vec2d::empty(),
		}
	}

	pub fn ready_time(&self) -> Time {
		INFINITE_TIME
	}

	// pub fn size(&self) -> Size {
	// 	self.terrain.size()
	// }

	// pub fn is_valid(&self, loc: Point) -> bool {
	// 	loc.x >= 0
	// 		&& loc.x < self.terrain.size().width
	// 		&& loc.y >= 0
	// 		&& loc.y < self.terrain.size().height
	// }

	// pub fn get(&self, loc: Point) -> &Terrain {
	// 	self.terrain.get(loc)
	// }

	pub fn on_event(&mut self, event: &Event, _queued: &mut QueuedEvents) {
		match event {
			Event::ResetLevel(name, size, terrain) => {
				self.name = name.to_string();
				self.terrain = Vec2d::new(*size, *terrain);
				self.tiles = Vec2d::new(*size, Level::DEFAULT_TILE);
			}
			Event::SetTerrain(loc, terrain) => {
				self.terrain.set(*loc, *terrain);
			}
			_ => (),
		}
	}

	/// screen_size is the number of tiles the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho if it is not within the player's LOS that info may be outdated).
	/// It can also be arbitrarily small though in that case the user may not be able
	/// to see all the tiles the player can.
	pub fn tiles(&mut self, screen_size: Size, player: &Player) -> Vec2d<Tile> {
		self.update_tiles(player);
		self.screen_tiles(screen_size, player)
		//self.invariant();		// TODO: probably want something like this
	}

	// Updates the tiles that are within the player's LOS.
	fn update_tiles(&mut self, player: &Player) {
		// The borrow checker won't allow us to grab a mutable reference to tiles in one closure and
		// another reference in the second closure so we'll figure out which cells are visible and
		// then update tiles.
		let player_loc = player.loc();
		let mut visible = FnvHashMap::default();
		let mut pov = POV {
			start: player_loc,
			size: self.terrain.size(),
			radius: 10, // TODO: depends on race?
			visible_tile: |loc| {
				let terrain = *self.terrain.get(loc);
				visible.insert(loc, terrain);
			},
			blocks_los: |loc| {
				let terrain = self.terrain.get(loc);
				matches!(terrain, Terrain::Wall) // TODO: do something better here
			},
		};
		pov.visit();

		self.tiles.apply(|loc, tile| {
			if let Some(terrain) = visible.get(&loc) {
				tile.character = if loc == player_loc {
					Some(CharTile { is_player: true })
				} else {
					None
				};
				tile.terrain = Some(*terrain);
				tile.visible = true;
			} else {
				tile.visible = false; // leave the other state as it was when it was last within the player's LOS
			}
		});

		// let mut visible_tiles = FnvHashMap::default();
		// {
		// 	let mut pov = POV {
		// 		start: player_loc,
		// 		size: self.terrain.size(),
		// 		radius: 10, // TODO: depends on race?
		// 		visit_tile: |loc| {
		// 			let ch = if loc == player_loc {
		// 				Some(CharTile { is_player: true })
		// 			} else {
		// 				None
		// 			};
		// 			let tile = Tile {
		// 				visible: true,
		// 				character: ch,
		// 				terrain: Some(*self.terrain.get(loc)),
		// 			};
		// 			all_tiles.insert(loc, tile);
		// 		},
		// 		blocks_los: |loc| {
		// 			let terrain = self.terrain.get(loc);
		// 			matches!(terrain, Terrain::Wall) // TODO: do something better here
		// 		},
		// 	};

		// 	pov.visit();
		// }

		// self.tiles.apply(|loc, tile| {
		// 	if let Some(cell) = all_tiles.get(&loc) {
		// 		tile.terrain = Some(cell.terrain);
		// 		tile.character = cell.character;
		// 		tile.visible = true;
		// 	} else {
		// 		tile.visible = false; // leave the other state as it was when it was last within the player's LOS
		// 	}
		// })
	}

	// Returns the subset of tiles that are rendered on the screen.
	fn screen_tiles(&self, screen_size: Size, player: &Player) -> Vec2d<Tile> {
		let mut tiles = Vec2d::new(screen_size, Level::DEFAULT_TILE);
		let player_loc = player.loc();
		let start_x = player_loc.x - screen_size.width / 2;
		let start_y = player_loc.y - screen_size.height / 2;
		for out_y in 0..screen_size.height {
			for out_x in 0..screen_size.width {
				let in_loc = Point::new(start_x + out_x, start_y + out_y);
				if in_loc.x >= 0
					&& in_loc.x < self.tiles.size().width
					&& in_loc.y >= 0 && in_loc.y < self.tiles.size().height
				{
					let tile = self.tiles.get(in_loc);
					let out_loc = Point::new(out_x, out_y);
					tiles.set(out_loc, tile.clone());
				}
			}
		}
		tiles
	}

	const DEFAULT_TILE: Tile = Tile {
		visible: false,
		character: None,
		terrain: None,
	};
}
