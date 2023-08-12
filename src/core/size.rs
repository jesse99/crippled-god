use file_scanner::Scanner;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Result, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Size {
        Size { width, height }
    }

    pub fn from_saved(scanner: &mut Scanner<File>) -> Option<Size> {
        match (scanner.next_int(), scanner.next_int()) {
            (Some(w), Some(h)) => Some(Size::new(w, h)),
            _ => None,
        }
    }

    // pub fn zero() -> Size {
    // 	Size {
    // 		width: 0,
    // 		height: 0,
    // 	}
    // }

    #[cfg(test)]
    pub fn area(self) -> i32 {
        self.width * self.height
    }

    pub fn write(&self, w: &mut BufWriter<File>) -> Result<()> {
        write!(w, "{} {}", self.width, self.height)
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}
