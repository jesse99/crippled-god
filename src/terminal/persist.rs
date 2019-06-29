
use super::backend::{Game, Message, Topic};
use super::colors::{self, Color};
use slog::Logger;
use std;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use termion;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

const SAVE_FILE_NAME: &str = "crippled-god.json";

pub fn has_saved_game() -> bool {
	match env::current_dir() {
		Ok(ref mut path) => {
			path.push(SAVE_FILE_NAME);
			path.as_path().is_file()
		}
		Err(_) => false,
	}
}

pub fn load_game(root_logger: &Logger) -> Result<Game, String> {
	let mut path = env::current_dir().map_err(|e| e.to_string())?;
	path.push(SAVE_FILE_NAME);

	let mut contents = String::new();
	let mut file = File::open(path).map_err(|e| e.to_string())?;
	file.read_to_string(&mut contents)
		.map_err(|e| e.to_string())?;

	let mut game: Game = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
	game.with_saved(root_logger);
	Ok(game)
}

pub fn save_game(game: &mut Game) {
	match do_save_game(game) {
		Ok(_) => game.add_message(Message {
			topic: Topic::NonGamePlay,
			text: "Saved game.".to_string(),
		}),
		Err(err) => game.add_message(Message {
			topic: Topic::Error,
			text: format!("Failed to save game: {}.", err).to_string(),
		}),
	}
}

fn do_save_game(game: &Game) -> Result<(), String> {
	let serialized =
		serde_json::to_string(game).map_err(|e| "serialization error, ".to_string() + &e.to_string())?;

	let mut file = File::create(SAVE_FILE_NAME).map_err(|e| e.to_string())?; // TODO: probably should put the file some place other than the working directory
	file.write_all(serialized.as_bytes())
		.map_err(|e| e.to_string())?;

	Ok(())
}
