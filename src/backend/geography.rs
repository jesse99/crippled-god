use super::*;
use rand;
use rand::Rng;
use std::fmt;

/// This is the background for a level. Features (like stairs), NPCs, and the player are maintained
/// as seperate data structures.
pub struct Geography {
	size: Size,
	terrain: Vec<Terrain>,
}

impl Geography {
	pub fn new() -> Geography {
		let width = 64;
		let height = 32;

		let mut terrain = vec![Terrain::Ground; width * height];

		// Add walls around the outside
		for x in 0..width {
			terrain[x] = Terrain::Wall;
			terrain[x + width * (height - 1)] = Terrain::Wall;
		}
		for y in 0..height {
			terrain[width * y] = Terrain::Wall;
			terrain[(width - 1) + width * y] = Terrain::Wall;
		}

		// Add a little lake in the middle.
		let x = width / 2;
		let y = height / 2 - 1;
		terrain[x + y * width] = Terrain::ShallowWater;
		terrain[x - 1 + (y + 1) * width] = Terrain::DeepWater;
		terrain[x + (y + 1) * width] = Terrain::DeepWater;
		terrain[x + 1 + (y + 1) * width] = Terrain::ShallowWater;
		terrain[x + (y + 2) * width] = Terrain::ShallowWater;

		// Add a short wall.
		let y = 8;
		terrain[x + 2 + y * width] = Terrain::Wall;
		terrain[x + 1 + y * width] = Terrain::Wall;
		terrain[x + 0 + y * width] = Terrain::Wall;
		terrain[x - 1 + y * width] = Terrain::Wall;
		terrain[x - 2 + y * width] = Terrain::Wall;

		let size = Size::new(width as i32, height as i32);
		Geography { size, terrain }
	}

	pub fn size(&self) -> Size {
		self.size
	}

	pub fn at(&self, loc: Location) -> Terrain {
		let i = loc.x + loc.y * self.size.width;
		self.terrain[i as usize]
	}

	/// Returns a randomized location that satisfies the predicate.
	pub fn find_loc_with<T>(&self, rng: &mut rand::XorShiftRng, predicate: T) -> Option<Location>
	where
		T: Fn(Terrain) -> bool,
	{
		let mut indexes: Vec<i32> = (0..self.size.width * self.size.height).collect();
		rng.shuffle(&mut indexes);

		for i in indexes.iter() {
			let terrain = self.terrain[*i as usize];
			if predicate(terrain) {
				let x = i % self.size.width;
				let y = i / self.size.width;
				return Some(Location::new(x, y));
			}
		}
		return None;
	}
}

impl fmt::Debug for Geography {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n")?;
		for y in 0..self.size.height {
			for x in 0..self.size.width {
				let loc = Location::new(x, y);
				write!(f, "{:?}", self.at(loc))?;
			}
			if y + 1 < self.size.height {
				write!(f, "\n")?;
			}
		}
		write!(f, "")
	}
}
