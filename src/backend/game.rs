use super::rng::*;
use super::scheduled::*;
use super::vec2::*;
use super::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum Key {
	UpArrow,
	DownArrow,
	LeftArrow,
	RightArrow,
	Char(char),
}

/// Used with Message.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Topic {
	/// An operation could not be completed.
	Error,

	/// Something that doesn't affect the game.
	NonGamePlay,

	/// Something has affected the player.
	Status,

	/// An operation was not completely successful.
	Warning,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
	pub topic: Topic,
	pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct Game {
	config: Config,
	level: Level,
	messages: VecDeque<Message>,
	rng: RNG,
	game_time: Time,
	running: bool,
}

impl Game {
	pub fn new(config_file: Result<String, String>, seed: u64) -> Game {
		let mut rng = RNG::new(seed);

		let mut messages = VecDeque::new();
		const VERSION: &'static str = env!("CARGO_PKG_VERSION");
		let greeting = format!("Welcome to the Crippled God version {}", VERSION);
		messages.push_back(Message {
			topic: Topic::NonGamePlay,
			text: greeting.to_string(),
		});

		let config = Config::default(config_file);
		let level = Level::new(&mut rng);
		let game_time = Time::zero();
		let running = true;
		let mut game = Game {
			config,
			level,
			messages,
			rng,
			game_time,
			running,
			// rng,
		};
		game.reload_config();
		game
	}

	pub fn with_saved(mut self) -> Game {
		self.running = true;
		self
	}

	pub fn config(&self) -> &Config {
		&self.config
	}

	pub fn messages(&self) -> &VecDeque<Message> {
		&self.messages
	}

	pub fn running(&self) -> bool {
		self.running
	}

	pub fn add_message(&mut self, topic: Topic, text: &str) {
		info!("{}", text);
		self.messages.push_back(Message {
			topic,
			text: text.to_string(),
		});
		while self.messages.len() > self.config.scroll_back {
			self.messages.pop_front();
		}
	}

	pub fn get_tiles(&mut self, screen_size: Size) -> Vec2<Tile> {
		self.level.get_tiles(screen_size)
	}

	/// If it's the player's turn to move then this just returns true. Otherwise it calls execute
	/// on the next Scheduled object which is ready.
	pub fn players_time_slice(&mut self) -> bool {
		let player_time = self.level.player().ready_time();
		match self.level.other_ready_time() {
			Some(other_time) if other_time < player_time => {
				self.game_time = other_time;
				self.level.execute_others(self.game_time, &mut self.rng);
				false
			}
			_ => {
				self.game_time = player_time;
				true
			}
		}
	}

	/// Returns false if the key was not handled.
	pub fn handle_key(&mut self, key: Key) -> bool {
		match key {
			Key::UpArrow | Key::Char('8') => move_player(self, 0, -1),
			Key::DownArrow | Key::Char('2') => move_player(self, 0, 1),
			Key::LeftArrow | Key::Char('4') => move_player(self, -1, 0),
			Key::RightArrow | Key::Char('6') => move_player(self, 1, 0),

			Key::Char('7') => move_player(self, -1, -1),
			Key::Char('9') => move_player(self, 1, -1),
			Key::Char('1') => move_player(self, -1, 1),
			Key::Char('3') => move_player(self, 1, 1),

			Key::Char('^') => {
				self.reload_config();
				true
			}
			Key::Char('q') => {
				self.running = false;
				true
			}
			_ => false,
		}
	}

	fn reload_config(&mut self) {
		let errors = self.config.reload();
		if errors.is_empty() {
			match self.config.config_path.clone() {
				Ok(path) => self.add_message(Topic::NonGamePlay, &format!("Loaded {}", path)),
				Err(err) => self.add_message(Topic::Warning, &err),
			}
		} else {
			for err in errors.iter() {
				self.add_message(Topic::Error, &format!("config error: {}", err));
			}
		}
	}
}

fn move_player(game: &mut Game, dx: i32, dy: i32) -> bool {
	let p = game.level.player_loc();
	let loc = Location::new(p.x + dx, p.y + dy);
	if game.level.player().can_move_to(&game.level, loc) {
		game.level.move_player(loc);
		if let Terrain::ShallowWater = game.level.get_terrain(loc) {
			game.add_message(Topic::Status, "You splash through the water.")
		}
		true
	} else {
		false
	}
}
