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

pub struct Game {
	pub level: Level,
	pub player: Player,
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

		let player = Player::new(Race::Human);
		let level = Level::new(&player, &mut rng);
		Game { level, player, rng }
	}
}
