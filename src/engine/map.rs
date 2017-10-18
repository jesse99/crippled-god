//#[macro_use]
use common;
use game;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub terrain: game::Terrain,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub default: Square,

    squares: Vec<Square>,
}

impl Map {
    // pub fn with_default(default: &Square) -> Self {
    //     Map {
    //         width: 0,
    //         height: 0,
    //         default: *default,
    //         squares: Vec::with_capacity(24 * 80),
    //     }
    // }

    pub fn with_size(width: i32, height: i32, default: &Square) -> Self {
        let mut squares = Vec::new();
        squares.resize((width * height) as usize, *default); // TODO: use resize_default once that's stable?
        Map {
            width,
            height,
            default: *default,
            squares,
        }
    }

    pub fn get_square(&self, x: i32, y: i32) -> &Square {
        match self.squares.get((x + y * self.width) as usize) {
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

    pub fn set_square(&mut self, x: i32, y: i32, s: &Square) {
        match self.squares.get_mut((x + y * self.width) as usize) {
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

    // /// Like set_square except that the map is grown if (x, y) is out of range.
    // pub fn force_square(&mut self, x: i32, y: i32, s: &Square) {
    //     while x >= self.width {
    //         self.extend_right();
    //     }
    //     while y >= self.height {
    //         self.extend_down();
    //     }
    //     match self.squares.get_mut(x + y * self.width) {
    //         Some(old) => *old = *s,
    //         None => panic!("extending should have prevented us from landing here"),
    //     };
    // }

    // fn extend_right(&mut self) {
    //     self.width += 1;
    //     if self.height == 0 {
    //         self.height = 1;
    //     }

    //     let x = self.width - 1; // need these to mollify the borrow checker
    //     let s = self.default;
    //     for y in 0..self.height {
    //         self.squares.insert(x + y * self.width, s);
    //     }
    // }

    // fn extend_down(&mut self) {
    //     self.height += 1;
    //     if self.width == 0 {
    //         self.width = 1;
    //     }
    //     for _ in 0..self.width {
    //         self.squares.push(self.default);
    //     }
    // }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s += &format!("{:?}", self.get_square(x, y).terrain);
            }
            if y + 1 < self.height {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}
