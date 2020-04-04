//! This module defines a Store struct which is used to manage a set of RDF style triplets.
//! The triplets contain a Subject, Predicate, and Object. For example,
//!    ("wolf-1", "description", "It has pointy teeth.")
//! The store encodes the entire game state and is operated upon by various components to
//! change state as the game is played and then to render the game. Note that there are
//! separate stores for each level.
use super::*;
use fnv::FnvHashMap;
use fnv::FnvHashSet;

/// This is used to identify an object within the game, eg an instance of an
/// NPC, the player, a location within the map, etc.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Subject(String);

impl Subject {
	/// Creates a unique subject, e.g. "player".
	pub fn new_unique(name: &str) -> Subject {
		// TODO: Could memoize these although that'd be copied so might not
		// be much of a win. Or maybe can have some sort of constant for the
		// most common unique subjects ("player" and "level").
		Subject(name.to_string())
	}

	/// Creates an instance of a subject, e.g. "wolf". Class is used by Store::
	/// iter_by_class.
	pub fn new_instance(store: &mut Store, class: &str, name: &str) -> Subject {
		Subject(store.instance_name(class, name))
	}
}

lazy_static! {
	pub static ref LEVEL: Subject = { Subject::new_unique("level") };
	pub static ref PLAYER: Subject = { Subject::new_unique("player") };
}

pub fn cell(loc: Point) -> Subject {
	Subject::new_unique(&format!("cell-{}-{}", loc.x, loc.y))
}

/// Used to form a relation between a Subject and an Object.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Predicate {
	Character,
	// Items,
	LastSeenChar,
	LastSeenTerrain,
	Loc,
	Name,
	Ready,
	Size,
	Terrain,
	Visible,
}

/// The value associated with a Subject and relation.
pub enum Object {
	Bool(bool),
	Point(Point),
	Ref(Subject),
	// Refs(Vec<Subject>),
	Size(Size),
	Str(String),
	Terrain(Terrain),
	Time(Time),
}

pub struct Store {
	count: u64,
	data: FnvHashMap<Subject, FnvHashMap<Predicate, Object>>,
	classes: FnvHashMap<String, FnvHashSet<Subject>>,
}

impl Store {
	// TODO: may want to replace this with a function that loads from a trait
	pub fn new() -> Store {
		Store {
			count: 0,
			data: FnvHashMap::default(),
			classes: FnvHashMap::default(),
		}
	}

	pub fn insert(&mut self, subject: &Subject, predicate: Predicate, object: Object) {
		// TODO: May want to do some profiling to see:
		// 1) If the store methods are a bottle neck.
		// 2) If a HashMap<Subject, [({Predicate, Object})] would be better.
		// 3) If a flat [(Subject, Predicate, Object would be better)].
		if let Some(inner) = self.data.get_mut(subject) {
			inner.insert(predicate, object); // usually we can just use the reference
		} else {
			let inner = self.data.entry(subject.clone()).or_default();
			inner.insert(predicate, object);
		}
	}

	pub fn remove(&mut self, subject: &Subject, predicate: Predicate) {
		if let Some(inner) = self.data.get_mut(subject) {
			inner.remove(&predicate);
		}
	}

	pub fn iter_by_class(self: &Store, class: &str) -> std::collections::hash_set::Iter<Subject> {
		self.classes.entry(class.to_string()).or_default().iter()
	}

	pub fn lookup_bool(&self, subject: &Subject, predicate: Predicate) -> Option<bool> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Bool(v)) = inner.get(&predicate) {
				Some(*v)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn lookup_pt(&self, subject: &Subject, predicate: Predicate) -> Option<Point> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Point(v)) = inner.get(&predicate) {
				Some(*v)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn lookup_ref(&self, subject: &Subject, predicate: Predicate) -> Option<Subject> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Ref(v)) = inner.get(&predicate) {
				Some(v.clone())
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn lookup_size(&self, subject: &Subject, predicate: Predicate) -> Option<Size> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Size(v)) = inner.get(&predicate) {
				Some(*v)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn lookup_time(&self, subject: &Subject, predicate: Predicate) -> Option<Time> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Time(v)) = inner.get(&predicate) {
				Some(*v)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn lookup_terrain(&self, subject: &Subject, predicate: Predicate) -> Option<Terrain> {
		if let Some(inner) = self.data.get(subject) {
			if let Some(Object::Terrain(v)) = inner.get(&predicate) {
				Some(*v)
			} else {
				None
			}
		} else {
			None
		}
	}

	fn instance_name(&mut self, class: &str, base: &str) -> String {
		let name = format!("{}-{}", base, self.count);
		self.count += 1;

		let inner = self.classes.entry(class.to_string()).or_default();
		inner.insert(Subject(name.clone()));

		name
	}
}
