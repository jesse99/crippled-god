use common;
use engine;
use rand;
use rand::SeedableRng;
use std;
use std::io::Write;
use terminal::render;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn termion_fatal_hook(message: &str) {
    let mut stdout = std::io::stdout();
    let _ = write!(
        stdout,
        "{}{}\n",
        termion::cursor::Restore,
        termion::cursor::Show
    );
    let _ = write!(stdout, "fatal error: {}\n", message);
}

pub fn run(seed: usize) {
    let (_, height) = termion::terminal_size().expect("couldn't get terminal size");
    // println!("width = {}, height = {}", width, height);

    common::set_fatal_hook(termion_fatal_hook);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let _ = write!(stdout, "\n{}{}", termion::cursor::Hide, termion::clear::All);

    let mut rng = rand::StdRng::from_seed(&[seed]);
    let mut map = create_map(&mut rng);
    let mut player_x = 5;
    let mut player_y = 5;

    render::render_map(&mut stdout, &map, player_x, player_y);
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Left => move_player(&map, &mut player_x, &mut player_y, -1, 0),
            termion::event::Key::Right => move_player(&map, &mut player_x, &mut player_y, 1, 0),
            termion::event::Key::Up => move_player(&map, &mut player_x, &mut player_y, 0, -1),
            termion::event::Key::Down => move_player(&map, &mut player_x, &mut player_y, 0, 1),
            termion::event::Key::Ctrl('r') => map = create_map(&mut rng),
            _ => {
                let _ = write!(stdout, "\x07");
            }
        };
        render::render_map(&mut stdout, &map, player_x, player_y);
    }

    let _ = write!(
        stdout,
        "\n{}{}{}",
        termion::cursor::Restore,
        termion::cursor::Show,
		termion::cursor::Goto(1, height)
    );
    stdout.flush().unwrap();
}

fn create_map(rng: &mut rand::StdRng) -> engine::Map {
    let mut stdout = std::io::stdout();
    let _ = write!(stdout, "\n{}{}", termion::cursor::Hide, termion::clear::All);	// TODO: we should be re-painting the entire screen so don't think we need this
    engine::generate_open(rng)	// TODO: should move the player too
}

fn move_player(map: &engine::Map, x: &mut i32, y: &mut i32, dx: i32, dy: i32) {
	if passable(map, *x + dx, *y + dy) {
		*x += dx;
		*y += dy;
	} else {
		let _ = write!(std::io::stdout(), "\x07");
	}
}

fn passable(map: &engine::Map, x: i32, y: i32) -> bool {
	let square = map.get_square(x, y);
	if square.terrain.passable() {
		if let Some(f) = square.feature {
			f.passable()
		} else {
			true
		}
	} else {
		false
	}
}