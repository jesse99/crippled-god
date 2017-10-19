use engine;
use game;
use std;
use std::io::Write;
use termion;

pub type RawTerminal = termion::raw::RawTerminal<std::io::Stdout>;

// See https://camo.githubusercontent.com/18622d6a234413cbc0aba27a09146797bf1eef4d/68747470733a2f2f692e696d6775722e636f6d2f4b696c72306d432e706e673f31
// and http://cng.seas.rochester.edu/CNG/docs/x11color.html
fn to_termion(color: game::Color) -> termion::color::AnsiValue {
    match color {
        game::Color::LightPink => termion::color::AnsiValue::rgb(5, 3, 5),
        game::Color::Pink => termion::color::AnsiValue::rgb(5, 2, 5),
        game::Color::Crimson => termion::color::AnsiValue::rgb(4, 0, 0),
        game::Color::LavenderBlush => termion::color::AnsiValue::rgb(5, 4, 5),
        game::Color::PaleVioletRed => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::HotPink => termion::color::AnsiValue::rgb(4, 0, 4),
        game::Color::DeepPink => termion::color::AnsiValue::rgb(4, 0, 2),
        game::Color::MediumVioletRed => termion::color::AnsiValue::rgb(2, 0, 1),
        game::Color::Orchid => termion::color::AnsiValue::rgb(3, 0, 5),
        game::Color::Thistle => termion::color::AnsiValue::grayscale(14),
        game::Color::Plum => termion::color::AnsiValue::rgb(3, 2, 5),
        game::Color::Violet => termion::color::AnsiValue::rgb(5, 1, 5),
        game::Color::Magenta => termion::color::AnsiValue::rgb(5, 0, 4),
        game::Color::Fuchsia => termion::color::AnsiValue::rgb(5, 0, 4),
        game::Color::DarkMagenta => termion::color::AnsiValue::rgb(1, 0, 1),
        game::Color::Purple => termion::color::AnsiValue::rgb(1, 0, 1),
        game::Color::MediumOrchid => termion::color::AnsiValue::rgb(2, 0, 3),
        game::Color::DarkViolet => termion::color::AnsiValue::rgb(2, 0, 1),
        game::Color::DarkOrchid => termion::color::AnsiValue::rgb(2, 0, 1),
        game::Color::Indigo => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::BlueViolet => termion::color::AnsiValue::rgb(2, 0, 1),
        game::Color::MediumPurple => termion::color::AnsiValue::rgb(1, 1, 5),
        game::Color::MediumSlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        game::Color::SlateBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        game::Color::DarkSlateBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::Lavender => termion::color::AnsiValue::grayscale(17),
        game::Color::GhostWhite => termion::color::AnsiValue::grayscale(22),
        game::Color::Blue => termion::color::AnsiValue::rgb(0, 0, 4),
        game::Color::MediumBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::MidnightBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::DarkBlue => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::Navy => termion::color::AnsiValue::rgb(0, 0, 1),
        game::Color::RoyalBlue => termion::color::AnsiValue::rgb(0, 0, 5),
        game::Color::CornflowerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        game::Color::LightSteelBlue => termion::color::AnsiValue::rgb(2, 4, 5),
        game::Color::LightSlateGray => termion::color::AnsiValue::grayscale(5),
        game::Color::SlateGray => termion::color::AnsiValue::grayscale(5),
        game::Color::DodgerBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        game::Color::AliceBlue => termion::color::AnsiValue::rgb(2, 5, 5),
        game::Color::SteelBlue => termion::color::AnsiValue::rgb(0, 2, 3),
        game::Color::LightSkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
        game::Color::SkyBlue => termion::color::AnsiValue::rgb(0, 4, 5),
        game::Color::DeepSkyBlue => termion::color::AnsiValue::rgb(0, 1, 5),
        game::Color::LightBlue => termion::color::AnsiValue::rgb(1, 5, 5),
        game::Color::PowderBlue => termion::color::AnsiValue::rgb(1, 5, 5),
        game::Color::CadetBlue => termion::color::AnsiValue::rgb(0, 1, 1),
        game::Color::Azure => termion::color::AnsiValue::rgb(4, 5, 5),
        game::Color::LightCyan => termion::color::AnsiValue::rgb(3, 5, 5),
        game::Color::PaleTurquoise => termion::color::AnsiValue::rgb(1, 5, 5),
        game::Color::Cyan => termion::color::AnsiValue::rgb(1, 3, 5),
        game::Color::Aqua => termion::color::AnsiValue::rgb(1, 3, 5),
        game::Color::DarkTurquoise => termion::color::AnsiValue::rgb(0, 2, 2),
        game::Color::DarkSlateGray => termion::color::AnsiValue::grayscale(1),
        game::Color::DarkCyan => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Teal => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::MediumTurquoise => termion::color::AnsiValue::rgb(0, 3, 2),
        game::Color::LightSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Turquoise => termion::color::AnsiValue::rgb(0, 3, 2),
        game::Color::Aquamarine => termion::color::AnsiValue::rgb(0, 5, 3),
        game::Color::MediumAquamarine => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::MediumSpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
        game::Color::MintCream => termion::color::AnsiValue::grayscale(23),
        game::Color::SpringGreen => termion::color::AnsiValue::rgb(0, 5, 0),
        game::Color::MediumSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::SeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Honeydew => termion::color::AnsiValue::rgb(2, 5, 2),
        game::Color::LightGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::PaleGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::DarkSeaGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::LimeGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Lime => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::ForestGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Green => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::DarkGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::Chartreuse => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::LawnGreen => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::GreenYellow => termion::color::AnsiValue::rgb(0, 4, 0),
        game::Color::DarkOliveGreen => termion::color::AnsiValue::rgb(0, 1, 0),
        game::Color::YellowGreen => termion::color::AnsiValue::rgb(1, 2, 0),
        game::Color::OliveDrab => termion::color::AnsiValue::rgb(0, 2, 0),
        game::Color::Beige => termion::color::AnsiValue::rgb(3, 3, 5),
        game::Color::LightGoldenrodYellow => termion::color::AnsiValue::rgb(3, 3, 5),
        game::Color::Ivory => termion::color::AnsiValue::rgb(5, 5, 4),
        game::Color::LightYellow => termion::color::AnsiValue::rgb(5, 5, 4),
        game::Color::Yellow => termion::color::AnsiValue::rgb(5, 5, 0),
        game::Color::Olive => termion::color::AnsiValue::rgb(2, 1, 0),
        game::Color::DarkKhaki => termion::color::AnsiValue::rgb(2, 1, 1),
        game::Color::LemonChiffon => termion::color::AnsiValue::rgb(4, 4, 2),
        game::Color::PaleGoldenrod => termion::color::AnsiValue::rgb(3, 3, 1),
        game::Color::Khaki => termion::color::AnsiValue::rgb(3, 3, 1),
        game::Color::Gold => termion::color::AnsiValue::rgb(4, 2, 0),
        game::Color::Cornsilk => termion::color::AnsiValue::rgb(4, 4, 2),
        game::Color::Goldenrod => termion::color::AnsiValue::rgb(2, 1, 0),
        game::Color::DarkGoldenrod => termion::color::AnsiValue::rgb(1, 1, 0),
        game::Color::FloralWhite => termion::color::AnsiValue::grayscale(23),
        game::Color::OldLace => termion::color::AnsiValue::grayscale(22),
        game::Color::Wheat => termion::color::AnsiValue::rgb(5, 3, 2),
        game::Color::Moccasin => termion::color::AnsiValue::rgb(5, 3, 2),
        game::Color::Orange => termion::color::AnsiValue::rgb(5, 1, 0),
        game::Color::PapayaWhip => termion::color::AnsiValue::rgb(5, 3, 3),
        game::Color::BlanchedAlmond => termion::color::AnsiValue::rgb(5, 3, 3),
        game::Color::NavajoWhite => termion::color::AnsiValue::rgb(5, 3, 2),
        game::Color::AntiqueWhite => termion::color::AnsiValue::rgb(5, 5, 5),
        game::Color::Tan => termion::color::AnsiValue::rgb(2, 1, 1),
        game::Color::BurlyWood => termion::color::AnsiValue::rgb(2, 1, 1),
        game::Color::Bisque => termion::color::AnsiValue::rgb(3, 1, 0),
        game::Color::DarkOrange => termion::color::AnsiValue::rgb(5, 1, 0),
        game::Color::Linen => termion::color::AnsiValue::grayscale(21),
        game::Color::Peru => termion::color::AnsiValue::rgb(1, 1, 0),
        game::Color::PeachPuff => termion::color::AnsiValue::rgb(3, 1, 0),
        game::Color::SandyBrown => termion::color::AnsiValue::rgb(3, 1, 0),
        game::Color::Chocolate => termion::color::AnsiValue::rgb(2, 0, 0),
        game::Color::SaddleBrown => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::Seashell => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::Sienna => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::LightSalmon => termion::color::AnsiValue::rgb(4, 1, 1),
        game::Color::Coral => termion::color::AnsiValue::rgb(4, 1, 1),
        game::Color::OrangeRed => termion::color::AnsiValue::rgb(4, 0, 0),
        game::Color::DarkSalmon => termion::color::AnsiValue::rgb(3, 1, 0),
        game::Color::Tomato => termion::color::AnsiValue::rgb(3, 0, 0),
        game::Color::MistyRose => termion::color::AnsiValue::rgb(5, 1, 1),
        game::Color::Salmon => termion::color::AnsiValue::rgb(4, 1, 0),
        game::Color::Snow => termion::color::AnsiValue::grayscale(23),
        game::Color::LightCoral => termion::color::AnsiValue::rgb(4, 1, 0),
        game::Color::RosyBrown => termion::color::AnsiValue::rgb(1, 1, 1),
        game::Color::IndianRed => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::Red => termion::color::AnsiValue::rgb(4, 0, 0),
        game::Color::Brown => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::FireBrick => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::DarkRed => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::Maroon => termion::color::AnsiValue::rgb(1, 0, 0),
        game::Color::White => termion::color::AnsiValue::grayscale(23),
        game::Color::WhiteSmoke => termion::color::AnsiValue::grayscale(19),
        game::Color::Gainsboro => termion::color::AnsiValue::grayscale(17),
        game::Color::LightGrey => termion::color::AnsiValue::grayscale(16),
        game::Color::Silver => termion::color::AnsiValue::grayscale(14),
        game::Color::DarkGray => termion::color::AnsiValue::grayscale(8),
        game::Color::Gray => termion::color::AnsiValue::grayscale(4),
        game::Color::DimGray => termion::color::AnsiValue::grayscale(3),
        game::Color::Black => termion::color::AnsiValue::grayscale(0),
    }
}

pub fn render_map(stdout: &mut RawTerminal, map: &engine::Map, player_x: i32, player_y: i32) {
    for y in 0..map.height {
        for x in 0..map.width {
            let square = map.get_square(x, y);
            let mut tile = square.terrain.render();
            if let Some(f) = square.feature {
                f.render(&mut tile);
            }
            if x == player_x && y == player_y {
                tile.symbol = '@';
                tile.fg = game::Color::Black
            };
            let _ = write!(
                stdout,
                "\n{}{}{}{}",
                termion::cursor::Goto((x + 1) as u16, (y + 1) as u16), // termion uses 1-based coordinates
                termion::color::Fg(to_termion(tile.fg)),
                termion::color::Bg(to_termion(tile.bg)),
                tile.symbol
            );
        }
    }
    stdout.flush().unwrap();
}
