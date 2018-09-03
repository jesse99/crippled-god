use std::fs::File;
// use std::io;
use std::io::prelude::*;
use toml;

pub struct TerminalConfig {
	pub num_lines: i32,
}

pub struct Config {
	pub scroll_back: usize,
	pub terminal: TerminalConfig,
	pub config_path: Option<String>,
}

impl TerminalConfig {
	pub fn new() -> TerminalConfig {
		TerminalConfig { num_lines: 8 }
	}
}

impl Config {
	pub fn default(config_path: Option<String>) -> Config {
		Config {
			scroll_back: 100,
			terminal: TerminalConfig::new(),
			config_path,
		}
	}

	pub fn reload(&mut self) -> Vec<String> {
		match self.do_reload() {
			Ok(_) => Vec::new(),
			Err(e) => e,
		}
	}

	fn do_reload(&mut self) -> Result<(), Vec<String>> {
		if let Some(path) = self.config_path.clone() {
			let contents = read_file(&path)?;
			let table = parse_string(&contents)?;
			let errors = self.process_game(table);
			if !errors.is_empty() {
				return Err(errors);
			}
		}
		Ok(())
	}

	fn process_game(&mut self, table: toml::value::Table) -> Vec<String> {
		let mut errors = Vec::new();

		for (key, value) in table.iter() {
			if key == "scroll_back" {
				match value {
					toml::Value::Integer(value) if *value > 0 => self.scroll_back = *value as usize,
					toml::Value::Integer(value) if *value == 0 => {
						errors.push("scroll_back value should not be zero.".to_string());
					}
					toml::Value::Integer(_) => {
						errors.push("scroll_back value is negative.".to_string())
					}
					_ => errors.push("scroll_back value is not an integer. ".to_string()),
				}
			} else if key == "terminal" {
				match value {
					toml::Value::Table(value) => self.process_terminal(&mut errors, value),
					_ => errors.push("terminal value is not table.".to_string()),
				}
			} else {
				errors.push(format!("{} is an unknown game entry.", key));
			}
		}

		errors
	}

	fn process_terminal(&mut self, errors: &mut Vec<String>, table: &toml::value::Table) {
		for (key, value) in table.iter() {
			if key == "num_lines" {
				match value {
					toml::Value::Integer(value) if *value > 0 => {
						self.terminal.num_lines = *value as i32
					}
					toml::Value::Integer(value) if *value == 0 => {
						errors.push("terminal.num_lines value should not be zero.".to_string());
					}
					toml::Value::Integer(_) => {
						errors.push("terminal.num_lines value is negative.".to_string())
					}
					_ => errors.push("terminal.num_lines value is not an integer.".to_string()),
				}
			} else {
				errors.push(format!("{} is an unknown game entry.", key));
			}
		}
	}
}

fn read_file(path: &str) -> Result<String, Vec<String>> {
	match File::open(path) {
		Ok(ref mut file) => {
			let mut contents = String::new();
			match file.read_to_string(&mut contents) {
				Ok(_) => Ok(contents),
				Err(err) => Err(vec![format!("{}", err)]),
			}
		}
		Err(err) => Err(vec![format!("{}", err)]),
	}
}

fn parse_string(text: &str) -> Result<toml::value::Table, Vec<String>> {
	match text.parse::<toml::Value>() {
		Ok(toml::Value::Table(t)) => Ok(t),
		Ok(_) => Err(vec!["config file isn't a table".to_string()]),
		Err(err) => Err(vec![format!("{}", err)]),
	}
}
