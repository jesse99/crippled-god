extern crate termion;

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::Write;

#[macro_use]
mod common;
mod engine;
mod game;

type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

// See https://camo.githubusercontent.com/18622d6a234413cbc0aba27a09146797bf1eef4d/68747470733a2f2f692e696d6775722e636f6d2f4b696c72306d432e706e673f31
// and http://cng.seas.rochester.edu/CNG/docs/x11color.html
fn to_termion(color: engine::Color) -> termion::color::AnsiValue {
    match color {
        engine::Color::LightPink => termion::color::AnsiValue::rgb(5, 3, 5),
        engine::Color::Pink => termion::color::AnsiValue::rgb(5, 2, 5),
        engine::Color::Crimson => termion::color::AnsiValue::rgb(4, 0, 0),
        engine::Color::LavenderBlush => termion::color::AnsiValue::rgb(5, 4, 5),
        engine::Color::PaleVioletRed => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::HotPink => termion::color::AnsiValue::rgb(4, 0, 4),
        engine::Color::DeepPink => termion::color::AnsiValue::rgb(4, 0, 2),
        engine::Color::MediumVioletRed => termion::color::AnsiValue::rgb(2, 0, 1),
        engine::Color::Orchid => termion::color::AnsiValue::rgb(3, 0, 5),
        engine::Color::Thistle => termion::color::AnsiValue::grayscale(14),
        engine::Color::Plum => termion::color::AnsiValue::rgb(3, 2, 5),
        engine::Color::Violet => termion::color::AnsiValue::rgb(5, 1, 5),
        engine::Color::Magenta => termion::color::AnsiValue::rgb(5, 0, 4),
        engine::Color::Fuchsia => termion::color::AnsiValue::rgb(5, 0, 4),
        engine::Color::DarkMagenta => termion::color::AnsiValue::rgb(1, 0, 1),
        engine::Color::Purple => termion::color::AnsiValue::rgb(1, 0, 1),
        engine::Color::MediumOrchid => termion::color::AnsiValue::rgb(2, 0, 3),
        engine::Color::DarkViolet => termion::color::AnsiValue::rgb(2, 0, 1),
        engine::Color::DarkOrchid => termion::color::AnsiValue::rgb(2, 0, 1),
        engine::Color::Indigo => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::BlueViolet => termion::color::AnsiValue::rgb(2, 0, 1),
        engine::Color::MediumPurple => termion::color::AnsiValue::rgb(1, 1, 5),
        engine::Color::MediumSlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        engine::Color::SlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        engine::Color::DarkSlateBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::Lavender => termion::color::AnsiValue::grayscale(17),
        engine::Color::GhostWhite => termion::color::AnsiValue::grayscale(22),
        engine::Color::Blue => termion::color::AnsiValue::rgb(0, 0, 4),
        engine::Color::MediumBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::MidnightBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::DarkBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::Navy => termion::color::AnsiValue::rgb(0, 0, 1),
        engine::Color::RoyalBlue => termion::color::AnsiValue::rgb(0, 0, 5),
        engine::Color::CornflowerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        engine::Color::LightSteelBlue => termion::color::AnsiValue::rgb(2, 4, 5),
        engine::Color::LightSlateGray => termion::color::AnsiValue::grayscale(5),
        engine::Color::SlateGray => termion::color::AnsiValue::grayscale(5),
        engine::Color::DodgerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        engine::Color::AliceBlue => termion::color::AnsiValue::rgb(2, 5, 5),
        engine::Color::SteelBlue => termion::color::AnsiValue::rgb(0, 2, 3),
        engine::Color::LightSkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
        engine::Color::SkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
        engine::Color::DeepSkyBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        engine::Color::LightBlue => termion::color::AnsiValue::rgb(1, 5, 5),
        engine::Color::PowderBlue => termion::color::AnsiValue::rgb(1, 5, 5),
        engine::Color::CadetBlue => termion::color::AnsiValue::rgb(0, 1, 1),
        engine::Color::Azure => termion::color::AnsiValue::rgb(4, 5, 5),
        engine::Color::LightCyan => termion::color::AnsiValue::rgb(3, 5, 5),
        engine::Color::PaleTurquoise => termion::color::AnsiValue::rgb(1, 5, 5),
        engine::Color::Cyan => termion::color::AnsiValue::rgb(1, 3, 5),
        engine::Color::Aqua => termion::color::AnsiValue::rgb(1, 3, 5),
        engine::Color::DarkTurquoise => termion::color::AnsiValue::rgb(0, 2, 2),
        engine::Color::DarkSlateGray => termion::color::AnsiValue::grayscale(1),
        engine::Color::DarkCyan => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Teal => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::MediumTurquoise => termion::color::AnsiValue::rgb(0, 3, 2),
        engine::Color::LightSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Turquoise => termion::color::AnsiValue::rgb(0, 3, 2),
        engine::Color::Aquamarine => termion::color::AnsiValue::rgb(0, 5, 3),
        engine::Color::MediumAquamarine => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::MediumSpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
        engine::Color::MintCream => termion::color::AnsiValue::grayscale(23),
        engine::Color::SpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
        engine::Color::MediumSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::SeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Honeydew => termion::color::AnsiValue::rgb(2, 5, 2),
        engine::Color::LightGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::PaleGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::DarkSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::LimeGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Lime => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::ForestGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Green => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::DarkGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::Chartreuse => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::LawnGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::GreenYellow => termion::color::AnsiValue::rgb(0, 4, 0),
        engine::Color::DarkOliveGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        engine::Color::YellowGreen => termion::color::AnsiValue::rgb(1, 2, 0),
        engine::Color::OliveDrab => termion::color::AnsiValue::rgb(0, 2, 0),
        engine::Color::Beige => termion::color::AnsiValue::rgb(3, 3, 5),
        engine::Color::LightGoldenrodYellow => termion::color::AnsiValue::rgb(3, 3, 5),
        engine::Color::Ivory => termion::color::AnsiValue::rgb(5, 5, 4),
        engine::Color::LightYellow => termion::color::AnsiValue::rgb(5, 5, 4),
        engine::Color::Yellow => termion::color::AnsiValue::rgb(5, 5, 0),
        engine::Color::Olive => termion::color::AnsiValue::rgb(2, 1, 0),
        engine::Color::DarkKhaki => termion::color::AnsiValue::rgb(2, 1, 1),
        engine::Color::LemonChiffon => termion::color::AnsiValue::rgb(4, 4, 2),
        engine::Color::PaleGoldenrod => termion::color::AnsiValue::rgb(3, 3, 1),
        engine::Color::Khaki => termion::color::AnsiValue::rgb(3, 3, 1),
        engine::Color::Gold => termion::color::AnsiValue::rgb(4, 2, 0),
        engine::Color::Cornsilk => termion::color::AnsiValue::rgb(4, 4, 2),
        engine::Color::Goldenrod => termion::color::AnsiValue::rgb(2, 1, 0),
        engine::Color::DarkGoldenrod => termion::color::AnsiValue::rgb(1, 1, 0),
        engine::Color::FloralWhite => termion::color::AnsiValue::grayscale(23),
        engine::Color::OldLace => termion::color::AnsiValue::grayscale(22),
        engine::Color::Wheat => termion::color::AnsiValue::rgb(5, 3, 2),
        engine::Color::Moccasin => termion::color::AnsiValue::rgb(5, 3, 2),
        engine::Color::Orange => termion::color::AnsiValue::rgb(5, 1, 0),
        engine::Color::PapayaWhip => termion::color::AnsiValue::rgb(5, 3, 3),
        engine::Color::BlanchedAlmond => termion::color::AnsiValue::rgb(5, 3, 3),
        engine::Color::NavajoWhite => termion::color::AnsiValue::rgb(5, 3, 2),
        engine::Color::AntiqueWhite => termion::color::AnsiValue::rgb(5, 5, 5),
        engine::Color::Tan => termion::color::AnsiValue::rgb(2, 1, 1),
        engine::Color::BurlyWood => termion::color::AnsiValue::rgb(2, 1, 1),
        engine::Color::Bisque => termion::color::AnsiValue::rgb(3, 1, 0),
        engine::Color::DarkOrange => termion::color::AnsiValue::rgb(5, 1, 0),
        engine::Color::Linen => termion::color::AnsiValue::grayscale(21),
        engine::Color::Peru => termion::color::AnsiValue::rgb(1, 1, 0),
        engine::Color::PeachPuff => termion::color::AnsiValue::rgb(3, 1, 0),
        engine::Color::SandyBrown => termion::color::AnsiValue::rgb(3, 1, 0),
        engine::Color::Chocolate => termion::color::AnsiValue::rgb(2, 0, 0),
        engine::Color::SaddleBrown => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::Seashell => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::Sienna => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::LightSalmon => termion::color::AnsiValue::rgb(4, 1, 1),
        engine::Color::Coral => termion::color::AnsiValue::rgb(4, 1, 1),
        engine::Color::OrangeRed => termion::color::AnsiValue::rgb(4, 0, 0),
        engine::Color::DarkSalmon => termion::color::AnsiValue::rgb(3, 1, 0),
        engine::Color::Tomato => termion::color::AnsiValue::rgb(3, 0, 0),
        engine::Color::MistyRose => termion::color::AnsiValue::rgb(5, 1, 1),
        engine::Color::Salmon => termion::color::AnsiValue::rgb(4, 1, 0),
        engine::Color::Snow => termion::color::AnsiValue::grayscale(23),
        engine::Color::LightCoral => termion::color::AnsiValue::rgb(4, 1, 0),
        engine::Color::RosyBrown => termion::color::AnsiValue::rgb(1, 1, 1),
        engine::Color::IndianRed => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::Red => termion::color::AnsiValue::rgb(4, 0, 0),
        engine::Color::Brown => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::FireBrick => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::DarkRed => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::Maroon => termion::color::AnsiValue::rgb(1, 0, 0),
        engine::Color::White => termion::color::AnsiValue::grayscale(23),
        engine::Color::WhiteSmoke => termion::color::AnsiValue::grayscale(19),
        engine::Color::Gainsboro => termion::color::AnsiValue::grayscale(17),
        engine::Color::LightGrey => termion::color::AnsiValue::grayscale(16),
        engine::Color::Silver => termion::color::AnsiValue::grayscale(14),
        engine::Color::DarkGray => termion::color::AnsiValue::grayscale(8),
        engine::Color::Gray => termion::color::AnsiValue::grayscale(4),
        engine::Color::DimGray => termion::color::AnsiValue::grayscale(3),
        engine::Color::Black => termion::color::AnsiValue::grayscale(0),
    }
}

fn render_map(
    stdout: &mut RawTerminal,
    map: &Vec<Vec<engine::Square>>,
    player_x: usize,
    player_y: usize,
) {
    let mut y = 1; // terminal coordinates are 1-based
    for row in map.iter() {
        for (x, s) in row.iter().enumerate() {
            let c = if x == player_x && y == player_y + 1 {
                '@'
            } else {
                s.symbol
            };
            let fcolor = if x == player_x && y == player_y + 1 {
                engine::Color::Black
            } else {
                s.fore_color
            };
            let _ = write!(
                stdout,
                "\n{}{}{}{}",
                termion::cursor::Goto((x + 1) as u16, y as u16),
                termion::color::Fg(to_termion(fcolor)),
                termion::color::Bg(to_termion(s.back_color)),
                c
            );
        }
        y += 1;
    }
    stdout.flush().unwrap();
}

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

fn main() {
    // let (width, height) = termion::terminal_size().expect("couldn't get terminal size");
    // println!("width = {}, height = {}", width, height);

    common::set_fatal_hook(termion_fatal_hook);

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let _ = write!(stdout, "\n{}{}", termion::cursor::Hide, termion::clear::All);

    let map = engine::build_map(game::TERRAIN);
    let mut player_x = 5;
    let mut player_y = 5;

    render_map(&mut stdout, &map, player_x, player_y);
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
        render_map(&mut stdout, &map, player_x, player_y);
    }

    let _ = write!(
        stdout,
        "\n{}{}",
        termion::cursor::Restore,
        termion::cursor::Show
    );
    stdout.flush().unwrap();
}
