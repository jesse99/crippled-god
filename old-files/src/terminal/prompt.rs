use super::colors::{self, Color};
// use super::colors::*;
// use super::console::*;
// use super::map::*;
// use super::persist::*;
// use backend;
use std;
use std::io::Write;
use termion;
use termion::input::TermRead;
// use termion::raw::IntoRawMode;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub struct Choice {
	keys: Vec<termion::event::Key>,
	text: String,
}

impl Choice {
	// pub fn new1(key1: termion::event::Key, text: &str) -> Choice {
	// 	Choice {
	// 		keys: vec![key1],
	// 		text: text.to_string(),
	// 	}
	// }

	pub fn new2(key1: termion::event::Key, key2: termion::event::Key, text: &str) -> Choice {
		Choice {
			keys: vec![key1, key2],
			text: text.to_string(),
		}
	}
}

/// Asks the user to select from one of several choices. Returns the index of the chosen choice.
pub fn prompt(stdout: &mut RawTerminal, choices: &[Choice]) -> usize {
	render_choices(stdout, &choices);
	stdout.flush().unwrap();

	let stdin = std::io::stdin();
	for c in stdin.keys() {
		if let Ok(key) = c {
			match match_choice(key, &choices) {
				Some(index) => {
					return index;
				}
				None => {
					// let fg = to_termion(Color::Red);
					// let _ = write!(
					// 	stdout,
					// 	"{}{}{}",
					// 	termion::cursor::Goto(1, 10),
					// 	termion::color::Fg(fg),
					// 	key_to_str(key),
					// );
					let _ = write!(stdout, "\x07");
					stdout.flush().unwrap();
				}
			}
		}
	}
	choices.len() + 1
}

fn match_choice(key: termion::event::Key, choices: &[Choice]) -> Option<usize> {
	for (i, choice) in choices.iter().enumerate() {
		if match_key(key, &choice.keys) {
			return Some(i);
		}
	}
	None
}

fn match_key(key: termion::event::Key, keys: &[termion::event::Key]) -> bool {
	let rhs = format!("{:?}", key);
	keys.iter().any(|k| format!("{:?}", k) == rhs) // note that termion keys cannot be compared directly...
}

fn render_choices(stdout: &mut RawTerminal, choices: &[Choice]) {
	let bg = colors::to_termion(Color::Black);
	let fg1 = colors::to_termion(Color::White);
	let fg2 = colors::to_termion(Color::LightBlue);

	let _ = write!(stdout, "{}{}", termion::color::Bg(bg), termion::clear::All);

	let mut y = 1;
	for choice in choices.iter() {
		let _ = write!(
			stdout,
			"{}{}{}{}: {}{}",
			termion::cursor::Goto(1, y),
			termion::color::Bg(bg),
			termion::color::Fg(fg1),
			keys_to_str(&choice.keys),
			termion::color::Fg(fg2),
			choice.text,
		);
		y += 1;
	}
}

fn keys_to_str(keys: &[termion::event::Key]) -> String {
	let parts: Vec<String> = keys.iter().map(|k| key_to_str(*k)).collect();
	parts.join(", ")
}

fn key_to_str(key: termion::event::Key) -> String {
	match key {
		termion::event::Key::Backspace => "backspace".to_string(),
		termion::event::Key::Left => "left-arrow".to_string(),
		termion::event::Key::Right => "right-arrow".to_string(),
		termion::event::Key::Up => "up-arrow".to_string(),
		termion::event::Key::Down => "down-arrow".to_string(),
		termion::event::Key::Home => "home".to_string(),
		termion::event::Key::End => "end".to_string(),
		termion::event::Key::PageUp => "page-up".to_string(),
		termion::event::Key::PageDown => "page-down".to_string(),
		termion::event::Key::Delete => "delete".to_string(),
		termion::event::Key::Insert => "insert".to_string(),
		termion::event::Key::F(n) => format!("F{}", n),
		termion::event::Key::Char('\n') => "return".to_string(),
		termion::event::Key::Char(c) => format!("{}", c),
		termion::event::Key::Alt(c) => format!("alt-{}", c),
		termion::event::Key::Ctrl(c) => format!("control-{}", c),
		termion::event::Key::Null => "null".to_string(),
		termion::event::Key::Esc => "escape".to_string(),
		termion::event::Key::__IsNotComplete => "in-complete".to_string(),
	}
}
