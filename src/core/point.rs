use file_scanner::Scanner;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

/// Location within the map.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn from_saved(scanner: &mut Scanner<File>) -> Option<Point> {
        match (scanner.next_int(), scanner.next_int()) {
            (Some(x), Some(y)) => Some(Point::new(x, y)),
            _ => None,
        }
    }

    /// top-left
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn write(&self, w: &mut BufWriter<File>) -> Result<()> {
        write!(w, "{} {}", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
