use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
	pub x: i32,
	pub y: i32,
}

impl Location {
	pub fn new(x: i32, y: i32) -> Location {
		Location { x, y }
	}

	// pub fn zero() -> Location {
	// 	Location { x: 0, y: 0 }
	// }

	// pub fn left(&self) -> Location {
	//  Location::new(self.x - 1, self.y)
	// }

	// pub fn right(&self) -> Location {
	//  Location::new(self.x + 1, self.y)
	// }

	// pub fn up(&self) -> Location {
	//  Location::new(self.x, self.y - 1)
	// }

	// pub fn down(&self) -> Location {
	//  Location::new(self.x, self.y + 1)
	// }

	// 	pub fn distance(self, rhs: Location) -> f64 {
	// 		let x1 = f64::from(self.x);
	// 		let y1 = f64::from(self.y);
	// 		let x2 = f64::from(rhs.x);
	// 		let y2 = f64::from(rhs.y);
	// 		let dx = x1 - x2;
	// 		let dy = y1 - y2;
	// 		(dx * dx + dy * dy).sqrt()
	// 	}
}

impl Add<Location> for Location {
	type Output = Location;

	fn add(self, other: Location) -> Location {
		Location::new(self.x + other.x, self.y + other.y)
	}
}

impl Sub<Location> for Location {
	type Output = Location;

	fn sub(self, other: Location) -> Location {
		Location::new(self.x - other.x, self.y - other.y)
	}
}

impl slog::Value for Location {
	fn serialize(
		&self,
		_: &slog::Record<'_>,
		key: slog::Key,
		serializer: &mut dyn slog::Serializer,
	) -> Result<(), slog::Error> {
		serializer.emit_arguments(key, &format_args!("({}, {})", self.x, self.y))
	}
}
