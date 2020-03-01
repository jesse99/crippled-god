use std::fs::File;
use std::io::prelude::*;
use toml;

// TODO: Add key bindings
#[derive(Deserialize, Serialize)]
pub struct TerminalConfig {
	pub num_lines: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
	pub scroll_back: usize,
	pub slow_asserts: bool,
	pub terminal: TerminalConfig,

	#[serde(skip)]
	pub config_path: Option<String>,
}

impl TerminalConfig {
	pub fn new() -> TerminalConfig {
		TerminalConfig { num_lines: 8 }
	}
}

impl Config {
	/// A valid config will always be returned. If the string is set then there was an error and
	/// the config value will be the default value. 
	pub fn new(config_path: Option<String>) -> (Config, Option<String>) {
		match config_path {
			Some(path) => match Config::load(&path) {
				Ok(config) => (config, None),
				Err(err) => (Config::default(), Some(err)),
			},
			None => (Config::default(), Some("No config path".to_string())),
		}
	}

	fn default() -> Config {
		Config {
			scroll_back: 100,
			slow_asserts: false,
			terminal: TerminalConfig::new(),
			config_path: None,
		}
	}

	fn load(path: &str) -> Result<Config, String> {
		let contents = read_file(path)?;
		let mut config = parse(&contents)?;
		config.config_path = Some(path.to_string());
		config.validate()?;
		Ok(config)
	}

	fn validate(self: &Config) -> Result<(), String> {
		let mut err = String::new();

		if self.scroll_back == 0 {
			err += "scroll_back should not be zero.";
		}

		if self.terminal.num_lines <= 0 {
			err += "terminal.num_lines should be larger than zero.";
		}

		if err.is_empty() {
			Ok(())
		} else {
			Err(err)
		}
	}
}

fn read_file(path: &str) -> Result<String, String> {
	match File::open(path) {
		Ok(ref mut file) => {
			let mut contents = String::new();
			match file.read_to_string(&mut contents) {
				Ok(_) => Ok(contents),
				Err(err) => Err(format!("{}", err)),
			}
		}
		Err(err) => Err(format!("{}", err)),
	}
}

fn parse(contents: &str) -> Result<Config, String> {
	match toml::from_str(contents) {
		Ok(config) => Ok(config),
		Err(err) => Err(err.to_string()),
	}
}

