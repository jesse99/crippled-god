use super::*;

#[derive(Clone)]
pub struct Vec2d<T> {
	size: Size,
	elements: Vec<T>,
}

pub struct Vec2dIter<'a, T: 'a> {
	index: usize,
	vector: &'a Vec2d<T>,
}

impl<T: Clone> Vec2d<T> {
	pub fn new(size: Size, default: T) -> Vec2d<T> {
		let elements = vec![default; size.area() as usize];
		Vec2d { size, elements }
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

	pub fn get_mut(&mut self, loc: Location) -> &mut T {
		let index = loc.x + loc.y * self.size.width;
		&mut self.elements[index as usize]
	}

	pub fn iter(&self) -> Vec2dIter<T> {
		Vec2dIter {
			index: 0,
			vector: self,
		}
	}

	// More elegant to use a mutable iterator here but that requires an unsafe block, see https://users.rust-lang.org/t/implementing-an-iterator-of-mutable-references/8671
	// pub fn apply<F: Fn(Location, &mut T)>(&mut self, mutate: F) {
	// 	for i in 0..self.elements.len() {
	// 		let x = (i % self.size.width as usize) as i32;
	// 		let y = (i / self.size.width as usize) as i32;
	// 		let loc = Location::new(x, y);
	// 		let val = self.elements.get_mut(i);
	// 		mutate(loc, val.unwrap());
	// 	}
	// }
}

impl<'a, T> Iterator for Vec2dIter<'a, T> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_iter() {
		let size = Size::new(2, 2);
		let mut v = Vec2d::new(size, '.');
		*v.get_mut(Location::new(1, 0)) = 'a';
		*v.get_mut(Location::new(0, 1)) = 'b';
		*v.get_mut(Location::new(1, 1)) = 'c';

		let mut locs = Vec::new();
		let mut values = Vec::new();
		for (loc, ch) in v.iter() {
			locs.push(loc);
			values.push(*ch);
		}

		assert_eq!(
			locs,
			vec![
				Location::new(0, 0),
				Location::new(1, 0),
				Location::new(0, 1),
				Location::new(1, 1)
			]
		);
		assert_eq!(values, vec!['.', 'a', 'b', 'c']);
	}
}
