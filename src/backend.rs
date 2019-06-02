mod internal;

use fnv::FnvHashMap;
use slog::Logger;
use std::hash::{Hash, Hasher};

use internal::vec2d::Vec2d;
use internal::level::Level;
use internal::systems::player_system;

pub use self::internal::entity::Entity;
// pub use self::internal::level::Level;
pub use self::internal::location::Location;
pub use self::internal::size::Size;
// pub use self::internal::systems::player_system;
pub use self::internal::terrain::Terrain;

#[derive(Clone)]
pub struct Tile {
	/// If true then the tile is currently within the player's field of view.
	/// If false then the tile state is as it was when the player last saw it.
	pub visible: bool,	

	/// Player or NPC.
	pub character: Option<Entity>,

	pub terrain: Terrain,
}

pub enum PlayerAction {
	DeltaEast,
	DeltaNorth,
	DeltaNorthEast,
	DeltaNorthWest,
	DeltaSouth,
	DeltaSouthEast,
	DeltaSouthWest,
	DeltaWest,
	Quit,
}

pub struct Game {
	level: Level,
	running: bool,
}

impl Game {
	pub fn new(logger: Logger) -> Game {	// TODO: should be taking a reference to a parent logger
		let level = Level::with_logger(logger);
		Game{level, running: true}
	}

	pub fn running(&self) -> bool {
		self.running
	}

	pub fn size(&self) -> Size {
		self.level.cells.size()
	}

	pub fn is_player(&self, entity: Entity) -> bool {
		entity == self.level.player
	}

	pub fn dispatch_action(&mut self, action: PlayerAction) {
		assert!(self.running);
		match action {
			PlayerAction::DeltaEast => player_system::delta_player_system(&mut self.level, Location::new(-1, 0)),
			PlayerAction::DeltaNorth => player_system::delta_player_system(&mut self.level, Location::new(0, -1)),
			PlayerAction::DeltaNorthEast => player_system::delta_player_system(&mut self.level, Location::new(-1, -1)),
			PlayerAction::DeltaNorthWest => player_system::delta_player_system(&mut self.level, Location::new(1, -1)),
			PlayerAction::DeltaSouth => player_system::delta_player_system(&mut self.level, Location::new(0, 1)),
			PlayerAction::DeltaSouthEast => player_system::delta_player_system(&mut self.level, Location::new(1, 1)),
			PlayerAction::DeltaSouthWest => player_system::delta_player_system(&mut self.level, Location::new(-1, 1)),
			PlayerAction::DeltaWest => player_system::delta_player_system(&mut self.level, Location::new(-1, 0)),
			PlayerAction::Quit => self.running = false,
		}
	}

	pub fn tiles(&self, screen_size: Size) -> Vec2d<Tile> {
		// TODO:
		// return the visible cells
		// return cells that were visible but are not now
		let player_loc = *(self.level.position_components.get(&self.level.player).unwrap());

		let mut tiles = Vec2d::new(screen_size, Game::DEFAULT_TILE);
		let start_x = player_loc.x - screen_size.width / 2;
		let start_y = player_loc.y - screen_size.height / 2;
		for out_y in 0..screen_size.height {
			for out_x in 0..screen_size.width {
				let in_loc = Location::new(start_x + out_x, start_y + out_y);
				if in_loc.x >= 0
					&& in_loc.x < self.size().width
					&& in_loc.y >= 0
					&& in_loc.y < self.size().height
				{
					let cell = self.level.cells.get(in_loc);
					let tile = Tile {visible: true, character: cell.character, terrain: cell.terrain};

					let out_loc = Location::new(out_x, out_y);
					tiles.set(out_loc, tile);
				}
			}
		}

		tiles
	}

	const DEFAULT_TILE: Tile = Tile {
		visible: false,
		character: None,
		terrain: Terrain::Blank,
	};
}
