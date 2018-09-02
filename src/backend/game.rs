// use super::geography::Geography;
use super::level::Level;
// use super::location::Location;
use super::player::*;
// use super::pov::visit_visible_cells;
// use super::size::Size;
// use super::terrain::BlocksLOS;
// use super::terrain::Terrain;
// use super::vec2::Vec2;
use rand;
use rand::SeedableRng;
// use std::collections::HashMap;
// use std::fmt;
use std::collections::VecDeque;

pub struct Game {
	pub level: Level,
	pub player: Player,
	pub output: VecDeque<String>,
	rng: rand::XorShiftRng,
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
		Game {
			level,
			player,
			output,
			rng,
		}
	}
}
