use common;
use engine;
use game;
use terminal::render;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std;
use std::io::Write;

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

pub fn run() {
    // let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
    // println!("width = {}, height = {}", width, height);

    common::set_fatal_hook(termion_fatal_hook);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let _ = write!(stdout, "\n{}{}", termion::cursor::Hide, termion::clear::All);

    let map = engine::build_map(game::TERRAIN);
    let mut player_x = 5;
    let mut player_y = 5;

    render::render_map(&mut stdout, &map, player_x, player_y);
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Char('q') => break,
            termion::event::Key::Left => player_x -= 1,
            termion::event::Key::Right => player_x += 1,
            termion::event::Key::Up => player_y -= 1,
            termion::event::Key::Down => player_y += 1,
            _ => {
                let _ = write!(stdout, "\x07");
            }
        };
        render::render_map(&mut stdout, &map, player_x, player_y);
    }

    let _ = write!(
        stdout,
        "\n{}{}",
        termion::cursor::Restore,
        termion::cursor::Show
    );
    stdout.flush().unwrap();
}
