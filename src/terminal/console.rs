//! The part of the terminal that shows the game's text output.
use super::colors::*;
use super::*;
use backend;
use std;
use std::io::Write;
use termion;

pub const NUM_OUTPUT_LINES: i32 = 8; // TODO: make this a config option

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

pub fn render_console(
	terminal_size: backend::Size,
	stdout: &mut RawTerminal,
	game: &backend::Game,
) {
	let bg = to_termion(Color::Black);
	let fg = to_termion(Color::White);

	let _ = write!(
		stdout,
		"{}{}",
		termion::color::Bg(bg),
		termion::color::Fg(fg)
	);

	let mut dy = 0;
	let width = terminal_size.width as u16;
	let height = terminal_size.height as u16;
	for line in game.output().iter().rev() {
		// we need to go backwards because when lines wrap we don't know how many screen lines they will take
		let strings = split_output(width as usize, line);
		for sub_str in strings.iter().rev() {
			render_line(width, height - dy, stdout, sub_str);
			dy += 1;
			if dy >= NUM_OUTPUT_LINES as u16 {
				break;
			}
		}
	}
}

fn render_line(width: u16, y: u16, stdout: &mut RawTerminal, text: &str) {
	let _ = write!(
		stdout,
		"{}{:width$}",
		termion::cursor::Goto(1, y),
		text,
		width = width as usize
	);
}

fn split_output(width: usize, line: &str) -> Vec<String> {
	let mut lines = Vec::new();

	let mut words: Vec<String> = line
		.split_whitespace()
		.map(|word| word.to_string())
		.collect();

	// The first line is always full width so we have to special case it.
	let line = collect_line(&mut words, width);
	lines.push(line);

	let indent = "   ".to_string();
	while !words.is_empty() {
		let line = collect_line(&mut words, width - indent.len());
		lines.push(indent.clone() + &line);
	}

	lines
}

fn collect_line(words: &mut Vec<String>, width: usize) -> String {
	let mut line = String::new();

	loop {
		if line.len() + words[0].len() > width {
			line += &words[0][0..(width - line.len())];
		} else {
			line += &words[0];
		}
		words.remove(0);

		if words.is_empty() || line.len() + words[0].len() >= width {
			break;
		}
		line += " ";
	}

	line
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_short_word() {
		let actual = split_output(4, "abc");
		let expected = vec!["abc".to_string()];
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_exact_word() {
		let actual = split_output(4, "abcd");
		let expected = vec!["abcd".to_string()];
		assert_eq!(actual, expected);
	}

	// We don't handle this case terribly well but that should be OK in practice.
	#[test]
	fn test_long_word() {
		let actual = split_output(4, "abcde");
		let expected = vec!["abcd".to_string()];
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_short_line() {
		let actual = split_output(6, "abc");
		let expected = vec!["abc".to_string()];
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_exact_line() {
		let actual = split_output(5, "ab cd");
		let expected = vec!["ab cd".to_string()];
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_really_long_line() {
		let actual = split_output(12, "The quick brown fox jumped over the lazy dog.");
		let expected = vec![
			"The quick".to_string(),
			"   brown fox".to_string(),
			"   jumped".to_string(),
			"   over the".to_string(),
			"   lazy dog.".to_string(),
		];
		assert_eq!(actual, expected);
	}
}
