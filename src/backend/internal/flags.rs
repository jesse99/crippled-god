
use fnv::FnvHashSet;
use std::hash::Hash;

// #[derive(Hash)]
pub struct Flags<T> {
	flags: FnvHashSet<T>,
}

impl<T: Copy + Eq + Hash> Flags<T> {
	pub fn new() -> Flags<T> {
		Flags {
			flags: FnvHashSet::default(),
		}
	}

	pub fn add(&mut self, flag: T) {
		if !self.has(flag) {
			self.flags.insert(flag);
		}
	}

	pub fn remove(&mut self, flag: T) {
		let _ = self.flags.remove(&flag);
	}

	pub fn has(&self, flag: T) -> bool {
		self.flags.contains(&flag)
	}
}
