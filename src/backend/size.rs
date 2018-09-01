use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Size {
	pub width: i32,
	pub height: i32,
}

impl Size {
	pub fn new(width: i32, height: i32) -> Size {
		Size { width, height }
	}

	pub fn area(&self) -> i32 {
		self.width * self.height
	}
}

impl fmt::Debug for Size {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.width, self.height)
	}
}
