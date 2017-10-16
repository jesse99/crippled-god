//#[macro_use]
use common;
use std::fmt;

/// These are the X11 color names (from http://cng.seas.rochester.edu/CNG/docs/x11color.html).
/// UIs are expected to handle these colors the best they can (terminals for example often
/// don't support true color so some of these colors will map to the same screen color).
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Color {
    LightPink,
    Pink,
    Crimson,
    LavenderBlush,
    PaleVioletRed,
    HotPink,
    DeepPink,
    MediumVioletRed,
    Orchid,
    Thistle,
    Plum,
    Violet,
    Magenta,
    Fuchsia,
    DarkMagenta,
    Purple,
    MediumOrchid,
    DarkViolet,
    DarkOrchid,
    Indigo,
    BlueViolet,
    MediumPurple,
    MediumSlateBlue,
    SlateBlue,
    DarkSlateBlue,
    Lavender,
    GhostWhite,
    Blue,
    MediumBlue,
    MidnightBlue,
    DarkBlue,
    Navy,
    RoyalBlue,
    CornflowerBlue,
    LightSteelBlue,
    LightSlateGray,
    SlateGray,
    DodgerBlue,
    AliceBlue,
    SteelBlue,
    LightSkyBlue,
    SkyBlue,
    DeepSkyBlue,
    LightBlue,
    PowderBlue,
    CadetBlue,
    Azure,
    LightCyan,
    PaleTurquoise,
    Cyan,
    Aqua,
    DarkTurquoise,
    DarkSlateGray,
    DarkCyan,
    Teal,
    MediumTurquoise,
    LightSeaGreen,
    Turquoise,
    Aquamarine,
    MediumAquamarine,
    MediumSpringGreen,
    MintCream,
    SpringGreen,
    MediumSeaGreen,
    SeaGreen,
    Honeydew,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    LimeGreen,
    Lime,
    ForestGreen,
    Green,
    DarkGreen,
    Chartreuse,
    LawnGreen,
    GreenYellow,
    DarkOliveGreen,
    YellowGreen,
    OliveDrab,
    Beige,
    LightGoldenrodYellow,
    Ivory,
    LightYellow,
    Yellow,
    Olive,
    DarkKhaki,
    LemonChiffon,
    PaleGoldenrod,
    Khaki,
    Gold,
    Cornsilk,
    Goldenrod,
    DarkGoldenrod,
    FloralWhite,
    OldLace,
    Wheat,
    Moccasin,
    Orange,
    PapayaWhip,
    BlanchedAlmond,
    NavajoWhite,
    AntiqueWhite,
    Tan,
    BurlyWood,
    Bisque,
    DarkOrange,
    Linen,
    Peru,
    PeachPuff,
    SandyBrown,
    Chocolate,
    SaddleBrown,
    Seashell,
    Sienna,
    LightSalmon,
    Coral,
    OrangeRed,
    DarkSalmon,
    Tomato,
    MistyRose,
    Salmon,
    Snow,
    LightCoral,
    RosyBrown,
    IndianRed,
    Red,
    Brown,
    FireBrick,
    DarkRed,
    Maroon,
    White,
    WhiteSmoke,
    Gainsboro,
    LightGrey,
    Silver,
    DarkGray,
    Gray,
    DimGray,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub symbol: char,
    pub back_color: Color,
    pub fore_color: Color,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub default: Square,

    squares: Vec<Square>,
}

impl Map {
    pub fn with_default(default: &Square) -> Self {
        Map {
            width: 0,
            height: 0,
            default: *default,
            squares: Vec::with_capacity(24 * 80),
        }
    }

    pub fn with_size(width: usize, height: usize, default: &Square) -> Self {
        let mut squares = Vec::new();
        squares.resize(width * height, *default); // TODO: use resize_default once that's stable?
        Map {
            width,
            height,
            default: *default,
            squares,
        }
    }

    pub fn get_square(&self, x: usize, y: usize) -> &Square {
        match self.squares.get(x + y * self.width) {
            Some(s) => s,
            None => {
                fatal_error!(
                    "({}, {}) is outside the map dimensions ({}, {})",
                    x,
                    y,
                    self.width,
                    self.height
                )
            }
        }
    }

    pub fn set_square(&mut self, x: usize, y: usize, s: &Square) {
        match self.squares.get_mut(x + y * self.width) {
            Some(old) => *old = *s,
            None => {
                fatal_error!(
                    "({}, {}) is outside the map dimensions ({}, {})",
                    x,
                    y,
                    self.width,
                    self.height
                )
            }
        };
    }

    /// Like set_square except that the map is grown if (x, y) is out of range.
    pub fn force_square(&mut self, x: usize, y: usize, s: &Square) {
        while x >= self.width {
            self.extend_right();
        }
        while y >= self.height {
            self.extend_down();
        }
        match self.squares.get_mut(x + y * self.width) {
            Some(old) => *old = *s,
            None => panic!("extending should have prevented us from landing here"),
        };
    }

    fn extend_right(&mut self) {
        self.width += 1;
        if self.height == 0 {
            self.height = 1;
        }

        let x = self.width - 1; // need these to mollify the borrow checker
        let s = self.default;
        for y in 0..self.height {
            self.squares.insert(x + y * self.width, s);
        }
    }

    fn extend_down(&mut self) {
        self.height += 1;
        if self.width == 0 {
            self.width = 1;
        }
        for _ in 0..self.width {
            self.squares.push(self.default);
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(self.get_square(x, y).symbol);
            }
            if y + 1 < self.height {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

pub fn build_map(text: &str) -> Map {
    let it = text.chars();
    let it = it.skip_while(|c| c.is_whitespace());

    let default = Square {
        symbol: ' ',
        back_color: Color::LightGrey,
        fore_color: Color::Black,
    };
    let mut map = Map::with_default(&default);
    let mut x = 0;
    let mut y = 0;
    for c in it {
        match c {
            '\n' => {
                fatal_error_if!(
                    y > 0 && x != map.width,
                    "row {}'s width doesn't match the widths of the earlier rows", // TODO: include the map origin
                    y + 1
                );
                x = 0;
                y += 1;
            }
            '#' => {
                map.force_square(
                    x,
                    y,
                    &Square {
                        symbol: ' ',
                        back_color: Color::SaddleBrown,
                        fore_color: Color::Black,
                    },
                );
                x += 1;
            }
            '=' => {
                map.force_square(
                    x,
                    y,
                    &Square {
                        symbol: ' ',
                        back_color: Color::Bisque,
                        fore_color: Color::Black,
                    },
                );
                x += 1;
            }
            'w' => {
                map.force_square(
                    x,
                    y,
                    &Square {
                        symbol: ' ',
                        back_color: Color::DodgerBlue,
                        fore_color: Color::Black,
                    },
                );
                x += 1;
            }
            _ => {
                map.force_square(
                    x,
                    y,
                    &Square {
                        symbol: c,
                        back_color: Color::LightGrey,
                        fore_color: Color::Black,
                    },
                );
                x += 1;
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(map: &mut Map, x: usize, y: usize, c: char) {
        map.set_square(
            x,
            y,
            &Square {
                symbol: c,
                back_color: Color::LightGrey,
                fore_color: Color::Black,
            },
        );
    }

    fn force(map: &mut Map, x: usize, y: usize, c: char) {
        map.force_square(
            x,
            y,
            &Square {
                symbol: c,
                back_color: Color::LightGrey,
                fore_color: Color::Black,
            },
        );
    }

    #[test]
    fn buildin() {
        let default = Square {
            symbol: '_',
            back_color: Color::LightGrey,
            fore_color: Color::Black,
        };
        let mut map = Map::with_size(3, 3, &default);
        set(&mut map, 0, 0, 'a');
        set(&mut map, 1, 0, 'b');
        set(&mut map, 2, 0, 'c');

        set(&mut map, 0, 1, 'd');
        set(&mut map, 1, 1, 'e');
        set(&mut map, 2, 1, 'f');

        set(&mut map, 0, 2, 'g');
        set(&mut map, 1, 2, 'h');
        set(&mut map, 2, 2, 'i');

        let s = format!("{:?}", map);
        assert_eq!(s, "abc\ndef\nghi");

        force(&mut map, 4, 1, 'X');
        let s = format!("{:?}", map);
        assert_eq!(s, "abc__\ndef_X\nghi__");

        force(&mut map, 2, 4, 'Y');
        let s = format!("{:?}", map);
        assert_eq!(s, "abc__\ndef_X\nghi__\n_____\n__Y__");
    }
}
