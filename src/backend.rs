mod internal;

use fnv::FnvHashMap;
use slog::Logger;
use std::collections::VecDeque;
// use std::hash::{Hash, Hasher};

use internal::level::Level;
use internal::pov::POV;
use internal::systems::player_system;
use internal::terrain::BlocksLOS;
use internal::vec2d::Vec2d;

pub use self::internal::entity::Entity;
// pub use self::internal::level::Level;
pub use self::internal::location::Location;
pub use self::internal::message::{Message, Topic};
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
	tiles: Vec2d<Tile>,
	messages: VecDeque<Message>,
	running: bool,
	logger: Logger,
}

impl Game {
	pub fn new(root_logger: &Logger) -> Game {
		let game_logger = root_logger.new(o!());
		let level = Level::with_logger(&game_logger);
		let size = level.cells.size();

		let mut messages = VecDeque::new();
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		let greeting = format!("Welcome to the Crippled God version {}", VERSION);
		messages.push_back(Message {
			topic: Topic::NonGamePlay,
			text: greeting.clone(),
			//	text: greeting.to_string(),
		});

		Game {
			level,
			tiles: Vec2d::new(size, Game::DEFAULT_TILE),
			messages,
			running: true,
			logger: game_logger,
		}
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

	pub fn messages(&self) -> &VecDeque<Message> {
		&self.messages
	}

	pub fn add_message(&mut self, message: Message) {
		info!(self.logger, "{}", message.text);
		self.messages.push_back(message);

		let scroll_back = 100;
		while self.messages.len() > scroll_back {
			//		while self.messages.len() > self.config.scroll_back {
			self.messages.pop_front();
		}
	}

	pub fn dispatch_action(&mut self, action: PlayerAction) {
		assert!(self.running);
		match action {
			PlayerAction::DeltaEast => {
				player_system::delta_player_system(self, Location::new(1, 0))
			}
			PlayerAction::DeltaNorth => {
				player_system::delta_player_system(self, Location::new(0, -1))
			}
			PlayerAction::DeltaNorthEast => {
				player_system::delta_player_system(self, Location::new(1, -1))
			}
			PlayerAction::DeltaNorthWest => {
				player_system::delta_player_system(self, Location::new(-1, -1))
			}
			PlayerAction::DeltaSouth => {
				player_system::delta_player_system(self, Location::new(0, 1))
			}
			PlayerAction::DeltaSouthEast => {
				player_system::delta_player_system(self, Location::new(1, 1))
			}
			PlayerAction::DeltaSouthWest => {
				player_system::delta_player_system(self, Location::new(-1, 1))
			}
			PlayerAction::DeltaWest => {
				player_system::delta_player_system(self, Location::new(-1, 0))
			}
			PlayerAction::Quit => self.running = false,
		}
	}

	/// screen_size is the number of tiles the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho that info may be outdated). It can also be arbitrarily small
	/// though in that case the user may not be able to see all the tiles the player can.
	pub fn tiles(&mut self, screen_size: Size) -> Vec2d<Tile> {
		self.update_tiles();
		self.screen_tiles(screen_size)

		// let player_loc = *(self
		// 	.level
		// 	.position_components
		// 	.get(&self.level.player)
		// 	.unwrap());

		// let mut tiles = Vec2d::new(screen_size, Game::DEFAULT_TILE);
		// let start_x = player_loc.x - screen_size.width / 2;
		// let start_y = player_loc.y - screen_size.height / 2;
		// for out_y in 0..screen_size.height {
		// 	for out_x in 0..screen_size.width {
		// 		let in_loc = Location::new(start_x + out_x, start_y + out_y);
		// 		if in_loc.x >= 0
		// 			&& in_loc.x < self.size().width
		// 			&& in_loc.y >= 0 && in_loc.y < self.size().height
		// 		{
		// 			let cell = self.level.cells.get(in_loc);
		// 			let tile = Tile {
		// 				visible: true,
		// 				character: cell.character,
		// 				terrain: cell.terrain,
		// 			};

		// 			let out_loc = Location::new(out_x, out_y);
		// 			tiles.set(out_loc, tile);
		// 		}
		// 	}
		// }

		// tiles
	}

	// Updates the tiles that are within the player's LOS.
	fn update_tiles(&mut self) {
		// The borrow checker won't allow us to grab a mutable reference to tiles in one closure and
		// another reference in the second closure so we need to figure out what we need to do before
		// we call apply.
		let player_loc = *(self
			.level
			.position_components
			.get(&self.level.player)
			.unwrap());
		let mut visible = FnvHashMap::default();
		{
			let mut pov = POV {
				start: player_loc,
				size: self.size(),
				radius: 10, // TODO: depends on race?
				visit_tile: |loc| {
					let cell = self.level.cells.get(loc);
					visible.insert(loc, cell.clone());
				},
				blocks_los: |loc| {
					let terrain = self.level.cells.get(loc).terrain;
					terrain.blocks_los()
				},
			};

			pov.visit();
		}

		let player = self.level.player;
		self.tiles.apply(|loc, tile| {
			if let Some(cell) = visible.get(&loc) {
				tile.terrain = cell.terrain;
				// tile.char_name = *ch;
				tile.character = if loc == player_loc {
					Some(player)
				} else {
					None
				};
				tile.visible = true;
			} else {
				tile.visible = false;
			}
		})
	}

	// Returns the subset of tiles that are rendered on the screen.
	fn screen_tiles(&self, screen_size: Size) -> Vec2d<Tile> {
		let mut tiles = Vec2d::new(screen_size, Game::DEFAULT_TILE);
		let player_loc = *(self
			.level
			.position_components
			.get(&self.level.player)
			.unwrap());
		let start_x = player_loc.x - screen_size.width / 2;
		let start_y = player_loc.y - screen_size.height / 2;
		for out_y in 0..screen_size.height {
			for out_x in 0..screen_size.width {
				let in_loc = Location::new(start_x + out_x, start_y + out_y);
				if in_loc.x >= 0
					&& in_loc.x < self.tiles.size().width
					&& in_loc.y >= 0 && in_loc.y < self.tiles.size().height
				{
					let tile = self.tiles.get(in_loc);
					let out_loc = Location::new(out_x, out_y);
					tiles.set(out_loc, tile.clone());
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
