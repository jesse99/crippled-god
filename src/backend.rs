mod internal;

use fnv::FnvHashMap;
use slog::Logger;
use std::collections::VecDeque;
// use std::hash::{Hash, Hasher};


use internal::config::Config;
use internal::level::{Level, Scheduled};
use internal::pov::POV;
use internal::rng::RNG;
use internal::systems::{ai_system, player_system};
use internal::terrain::BlocksLOS;
// use internal::time;
use internal::vec2d::Vec2d;

pub use self::internal::character::Species;
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

	/// None if the player has never seen the tile.
	pub terrain: Option<Terrain>,
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
	pub config: Config,

	level: Level,
	tiles: Vec2d<Tile>,
	messages: VecDeque<Message>,
	running: bool,
	logger: Logger,
}

impl Game {
	pub fn new(config_path: Option<String>, root_logger: &Logger, seed: u64) -> Game {
		let game_logger = root_logger.new(o!());
		let rng = RNG::new(seed);
		let (config, err) = Config::new(config_path);
		let level = Level::with_logger(&game_logger, rng, config.slow_asserts); // TODO: on reload config need to reset level.slow_asserts
		let size = level.cells.size();

		let mut messages = VecDeque::new();
		const VERSION: &str = env!("CARGO_PKG_VERSION");
		let greeting = format!("Welcome to the Crippled God version {}", VERSION);
		messages.push_back(Message {
			topic: Topic::NonGamePlay,
			text: greeting.clone(),
		});

		if let Some(err) = err {
			messages.push_back(Message {
				topic: Topic::Error,
				text: err,
			});
		}

		if config.slow_asserts {
			messages.push_back(Message {
				topic: Topic::NonGamePlay,
				text: "Slow asserts are enabled!".to_string(),
			});
		}

		let game = Game {
			config,
			level,
			tiles: Vec2d::new(size, Game::DEFAULT_TILE),
			messages,
			running: true,
			logger: game_logger,
		};
		game.invariant();
		game
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

	pub fn get_species(&self, entity: Entity) -> Species {
		let c = self
			.level
			.character_components
			.get(&entity)
			.expect(&format!("expeted to find {:?}", entity));
		c.species
	}

	pub fn execute_others(&mut self) {
		while !self.player_ready() && !self.level.scheduled.is_empty() {
			let s = self
				.level
				.scheduled
				.pop()
				.expect("Should have found a scheduled non-player entity");
			if let Some(duration) = ai_system::act(self, s.entity) {
				assert!(duration.0 > 0);
				self.level.scheduled.push(Scheduled {
					entity: s.entity,
					time: s.time + duration,
				});
			} else {
				self.level.remove_entity(s.entity);
			}
		}
		self.invariant();
		self.level.invariant();
	}

	fn player_ready(&self) -> bool {
		if let Some(s) = self.level.scheduled.peek() {
			s.entity == self.level.player
		} else {
			false
		}
	}

	// pub fn messages(&self) -> &VecDeque<Message> {
	// 	&self.messages
	// }

	pub fn add_message(&mut self, message: Message) {
		info!(self.logger, "{}", message.text);
		self.messages.push_back(message);

		let scroll_back = 100;
		while self.messages.len() > scroll_back {
			//		while self.messages.len() > self.config.scroll_back {
			self.messages.pop_front();
		}
		self.invariant();
	}

	pub fn dispatch_action(&mut self, action: PlayerAction) {
		assert!(self.running);

		let s = self
			.level
			.scheduled
			.pop()
			.expect("Should have found a scheduled player entity");
		assert!(s.entity == self.level.player);

		let duration = match action {
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
			PlayerAction::Quit => {
				self.running = false;
				None
			}
		};

		if let Some(d) = duration {
			assert!(d.0 > 0); // duration shouldn't be negative
			assert!(d.0 < 10_000 * 100); // also shouldn't be crazy big
			self.level.scheduled.push(Scheduled {
				entity: s.entity,
				time: s.time + d,
			});
		} else {
			self.level.scheduled.push(s); // we'll push even when quitting so that the invariant doesn't complain
		}
		self.invariant();
		self.level.invariant();
	}

	/// screen_size is the number of tiles the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho if it is not within the player's LOS that info may be outdated).
	/// It can also be arbitrarily small though in that case the user may not be able
	/// to see all the tiles the player can.
	pub fn tiles(&mut self, screen_size: Size) -> Vec2d<Tile> {
		self.update_tiles();
		let tiles = self.screen_tiles(screen_size);
		self.invariant();
		tiles
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

		self.tiles.apply(|loc, tile| {
			if let Some(cell) = visible.get(&loc) {
				tile.terrain = Some(cell.terrain);
				tile.character = cell.character;
				tile.visible = true;
			} else {
				tile.visible = false; // leave the other state as it was when it was last within the player's LOS
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
		terrain: None,
	};

	#[cfg(debug_assertions)]
	fn invariant(&self) {
		assert!(self.level.cells.size() == self.tiles.size());

		if self.config.slow_asserts {
			let mut entities = FnvHashMap::default();
			for (_, tile) in self.tiles.iter() {
				if let Some(entity) = tile.character {
					let count = entities.entry(entity).or_insert(0);
					*count += 1;
				}
			}

			for (entity, count) in entities {
				assert!(
					count == 1,
					"{:?} appears {} times in game.tiles",
					entity,
					count
				);
			}
		}
	}
}
