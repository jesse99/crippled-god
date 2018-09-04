use super::colors::*;
// use super::map::*;
use backend;
use serde_json;
use std;
use std::env;
use std::fs::File;
// use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use termion;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

const SAVE_FILE_NAME: &'static str = "crippled-god.json";

pub fn has_saved_game() -> bool {
	match env::current_dir() {
		Ok(ref mut path) => {
			path.push(SAVE_FILE_NAME);
			path.as_path().is_file()
		}
		Err(_) => false,
	}
}

pub fn load_game() -> Result<backend::Game, String> {
	let mut path = env::current_dir().map_err(|e| e.to_string())?;
	path.push(SAVE_FILE_NAME);

	let mut file = File::open(path).map_err(|e| e.to_string())?;

	let mut contents = String::new();
	file.read_to_string(&mut contents)
		.map_err(|e| e.to_string())?;

	let game: backend::Game = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
	Ok(game)
}

pub fn save_game(stdout: &mut RawTerminal, game: &backend::Game) {
	if let Err(err) = do_save_game(game) {
		let color = to_termion(Color::Red);
		let _ = write!(
			stdout,
			"{}{}{}",
			termion::cursor::Goto(1, 2),
			termion::color::Fg(color),
			err
		);
	}
}

fn do_save_game(game: &backend::Game) -> Result<(), String> {
	let serialized = serde_json::to_string(game).map_err(|e| e.to_string())?;

	let mut file = File::create(SAVE_FILE_NAME).map_err(|e| e.to_string())?; // TODO: probably should put the file some place other than the working directory
	file.write_all(serialized.as_bytes())
		.map_err(|e| e.to_string())?;

	Ok(())
}
