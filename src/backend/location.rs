use std::fmt;

/// Note that (0, 0) is the top left of the level.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Location {
	pub x: i32,
	pub y: i32,
}

impl Location {
	pub fn new(x: i32, y: i32) -> Location {
		Location { x, y }
	}

	// pub fn left(&self) -> Location {
	// 	Location::new(self.x - 1, self.y)
	// }

	// pub fn right(&self) -> Location {
	// 	Location::new(self.x + 1, self.y)
	// }

	// pub fn up(&self) -> Location {
	// 	Location::new(self.x, self.y - 1)
	// }

	// pub fn down(&self) -> Location {
	// 	Location::new(self.x, self.y + 1)
	// }

	#[allow(dead_code)]
	pub fn distance(&self, rhs: Location) -> f64 {
		let x1 = self.x as f64;
		let y1 = self.y as f64;
		let x2 = rhs.x as f64;
		let y2 = rhs.y as f64;
		let dx = x1 - x2;
		let dy = y1 - y2;
		(dx * dx + dy * dy).sqrt()
	}
}

impl fmt::Debug for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}
