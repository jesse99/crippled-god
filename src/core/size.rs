#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Size {
	pub width: i32,
	pub height: i32,
}

impl Size {
	pub fn new(width: i32, height: i32) -> Size {
		Size { width, height }
	}

	pub fn zero() -> Size {
		Size {
			width: 0,
			height: 0,
		}
	}

	pub fn area(self: &Size) -> i32 {
		self.width * self.height
	}
}
