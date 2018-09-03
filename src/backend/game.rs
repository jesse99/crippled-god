use super::vec2::*;
use super::*;
use rand;
use rand::SeedableRng;
use std::collections::VecDeque;

pub enum Key {
	UpArrow,
	DownArrow,
	LeftArrow,
	RightArrow,
	Char(char),
}

pub struct Game {
	config: Config,
	level: Level,
	player: Player,
	output: VecDeque<String>,
	// rng: rand::XorShiftRng,
	running: bool,
}

impl Game {
	pub fn new(config_file: Option<String>, seed: usize) -> Game {
		let seed = [
			((seed >> 24) & 0xFF) as u8,
			((seed >> 16) & 0xFF) as u8,
			((seed >> 8) & 0xFF) as u8,
			(seed & 0xFF) as u8,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
		];
		let mut rng = rand::XorShiftRng::from_seed(seed);

		let mut output = VecDeque::new();
		const VERSION: &'static str = env!("CARGO_PKG_VERSION");
		let greeting = format!("Welcome to the Crippled God version {}", VERSION);
		output.push_back(greeting.to_string());

		let config = Config::default(config_file);
		let player = Player::new(Race::Human);
		let level = Level::new(&player, &mut rng);
		let running = true;
		let mut game = Game {
			config,
			level,
			player,
			output,
			running,
			// rng,
		};
		game.reload_config();
		game
	}

	pub fn config(&self) -> &Config {
		&self.config
	}

	pub fn output(&self) -> &VecDeque<String> {
		&self.output
	}

	pub fn running(&self) -> bool {
		self.running
	}

	// TODO: COlor code these. Or maybe use a topic.
	pub fn add_message(&mut self, message: &str) {
		info!("{}", message);
		self.output.push_back(message.to_string());
		while self.output.len() > self.config.scroll_back {
			self.output.pop_front();
		}
	}

	pub fn get_cells(&mut self, screen_size: Size) -> Vec2<Cell> {
		self.level.get_cells(&self.player, screen_size)
	}

	/// Returns false if the key was not handled.
	pub fn handle_key(&mut self, key: Key) -> bool {
		match key {
			Key::UpArrow => move_player(self, 0, -1),
			Key::DownArrow => move_player(self, 0, 1),
			Key::LeftArrow => move_player(self, -1, 0),
			Key::RightArrow => move_player(self, 1, 0),
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
				Some(path) => self.add_message(&format!("Loaded {}", path)),
				None => self.add_message("No config file"),
			}
		} else {
			for err in errors.iter() {
				self.add_message(&format!("config error: {}", err));
			}
		}
	}
}

fn move_player(game: &mut Game, dx: i32, dy: i32) -> bool {
	let p = game.level.player_loc();
	let loc = Location::new(p.x + dx, p.y + dy);
	if game.player.can_move_to(&game.level, loc) {
		game.level.move_player(&game.player, loc);
		if let Terrain::ShallowWater = game.level.geography().at(loc) {
			game.add_message("You splash through the water.")
		}
		true
	} else {
		false
	}
}
