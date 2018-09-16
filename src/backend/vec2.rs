use super::*;
use std::fmt;

/// Two dimensional vector.
#[derive(Clone, Deserialize, Serialize)]
pub struct Vec2<T> {
	size: Size,
	elements: Vec<T>,
}

pub struct Vec2Iter<'a, T: 'a> {
	index: usize,
	vector: &'a Vec2<T>,
}

impl<T: Clone> Vec2<T> {
	pub fn new(size: Size, default: T) -> Vec2<T> {
		let elements = vec![default; size.area() as usize];
		Vec2 { size, elements }
	}

	pub fn size(&self) -> Size {
		self.size
	}

	pub fn set(&mut self, loc: Location, value: T) {
		let index = loc.x + loc.y * self.size.width;
		self.elements[index as usize] = value;
	}

	pub fn get(&self, loc: Location) -> &T {
		let index = loc.x + loc.y * self.size.width;
		&self.elements[index as usize]
	}

	#[allow(dead_code)]
	pub fn get_mut(&mut self, loc: Location) -> &mut T {
		let index = loc.x + loc.y * self.size.width;
		&mut self.elements[index as usize]
	}

	pub fn iter(&self) -> Vec2Iter<T> {
		Vec2Iter {
			index: 0,
			vector: self,
		}
	}

	// More elegant to use a mutable iterator here but that requires an unsafe block, see https://users.rust-lang.org/t/implementing-an-iterator-of-mutable-references/8671
	pub fn apply<F: Fn(Location, &mut T)>(&mut self, mutate: F) {
		for i in 0..self.elements.len() {
			let x = (i % self.size.width as usize) as i32;
			let y = (i / self.size.width as usize) as i32;
			let loc = Location::new(x, y);
			let val = self.elements.get_mut(i);
			mutate(loc, val.unwrap());
		}
	}
}

impl<'a, T> Iterator for Vec2Iter<'a, T> {
	type Item = (Location, &'a T);

	fn next(&mut self) -> Option<(Location, &'a T)> {
		if self.index < self.vector.elements.len() {
			let i = self.index;
			self.index += 1;

			let x = (i % self.vector.size.width as usize) as i32;
			let y = (i / self.vector.size.width as usize) as i32;
			let loc = Location::new(x, y);
			let val = self.vector.elements.get(i);
			Some((loc, val.unwrap()))
		} else {
			None
		}
	}
}

impl<T: Clone + fmt::Debug> fmt::Debug for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f)?;
		for y in 0..self.size.height {
			for x in 0..self.size.width {
				let loc = Location::new(x, y);
				write!(f, "{:?}", self.get(loc))?;
			}
			if y + 1 < self.size.height {
				writeln!(f)?;
			}
		}
		write!(f, "")
	}
}

impl<T: Clone + fmt::Display> fmt::Display for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f)?;
		for y in 0..self.size.height {
			for x in 0..self.size.width {
				let loc = Location::new(x, y);
				write!(f, "{}", self.get(loc))?;
			}
			if y + 1 < self.size.height {
				writeln!(f)?;
			}
		}
		write!(f, "")
	}
}
