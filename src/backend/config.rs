// use super::vec2::*;
// use super::*;
// use rand;
// use rand::SeedableRng;
// use std::collections::VecDeque;

pub struct TerminalConfig {
	pub num_lines: i32,
}

pub struct Config {
	pub scroll_back: usize,
	pub terminal: TerminalConfig,
}

impl TerminalConfig {
	pub fn new() -> TerminalConfig {
		TerminalConfig { num_lines: 8 }
	}
}

impl Config {
	pub fn new() -> Config {
		Config {
			scroll_back: 100,
			terminal: TerminalConfig::new(),
		}
	}
}
