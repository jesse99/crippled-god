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
	level: Level,
	player: Player,
	output: VecDeque<String>,
	// rng: rand::XorShiftRng,
	running: bool,
}

impl Game {
	pub fn new(seed: usize) -> Game {
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

		output.push_back("This is line number 1".to_string());
		output.push_back("This is line number 2".to_string());
		output.push_back("This is line number 3".to_string());
		output.push_back("This is line number 4".to_string());
		output.push_back("This is line number 5".to_string());
		output.push_back("This is line number 6".to_string());
		output.push_back("Line 7: the quick brown fox jumped over the lazy dog and landed on the moon in a huff. It then rested for a spell and jumped not quite as high.".to_string());

		let player = Player::new(Race::Human);
		let level = Level::new(&player, &mut rng);
		let running = true;
		Game {
			level,
			player,
			output,
			running,
			// rng,
		}
	}

	pub fn output(&self) -> &VecDeque<String> {
		&self.output
	}

	pub fn running(&self) -> bool {
		self.running
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
			Key::Char('q') => {
				self.running = false;
				true
			}
			_ => false,
		}
	}
}

fn move_player(game: &mut Game, dx: i32, dy: i32) -> bool {
	let p = game.level.player_loc();
	let loc = Location::new(p.x + dx, p.y + dy);
	if game.player.can_move_to(&game.level, loc) {
		game.level.move_player(&game.player, loc);
		true
	} else {
		false
	}
}
