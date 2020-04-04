//! This module defines a Store struct which is used to manage a set of RDF style triplets.
//! The triplets contain a Subject, Predicate, and Object. For example,
//!    ("wolf-1", "description", "It has pointy teeth.")
//! The store encodes the entire game state and is operated upon by various components to
//! change state as the game is played and then to render the game. Note that there are
//! separate stores for each level.
use super::*;
use fnv::FnvHashMap;
use fnv::FnvHashSet;
use slog::Logger;
use std::fmt;

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

impl fmt::Display for Subject {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "\"{}\"", self.0)
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
#[derive(Debug)]
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

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Object::Bool(true) => write!(f, "true"),
			Object::Bool(false) => write!(f, "false"),
			Object::Point(v) => write!(f, "{}", v),
			Object::Ref(v) => write!(f, "{}", v),
			Object::Size(v) => write!(f, "{}", v),
			Object::Str(v) => write!(f, "\"{}\"", v),
			Object::Terrain(v) => write!(f, "{}", v),
			Object::Time(v) => write!(f, "{}", v),
		}
	}
}

#[derive(Debug)]
struct Triplet<'a> {
	subject: &'a Subject,
	predicate: &'a Predicate,
	object: &'a Object,
}

impl<'a> Triplet<'a> {
	fn new(subject: &'a Subject, predicate: &'a Predicate, object: &'a Object) -> Triplet<'a> {
		Triplet {
			subject,
			predicate,
			object,
		}
	}
}

impl<'a> fmt::Display for Triplet<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"({}, {:?}, {})",
			self.subject, self.predicate, self.object
		)
	}
}

pub struct Store {
	count: u64,
	data: FnvHashMap<Subject, FnvHashMap<Predicate, Object>>,
	classes: FnvHashMap<String, FnvHashSet<Subject>>,
	empty: FnvHashSet<Subject>,
	logger: Logger,
}

impl Store {
	// TODO: may want to replace this with a function that loads from a trait
	pub fn new(root_logger: &Logger) -> Store {
		Store {
			count: 0,
			data: FnvHashMap::default(),
			classes: FnvHashMap::default(),
			empty: FnvHashSet::default(),
			logger: root_logger.new(o!()),
		}
	}

	pub fn insert(&mut self, subject: &Subject, predicate: Predicate, object: Object) {
		trace!(self.logger, "inserting"; "triplet" => %Triplet::new(subject, &predicate, &object));

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
			if let Some(object) = inner.remove(&predicate) {
				trace!(self.logger, "removed"; "triplet" => %Triplet::new(subject, &predicate, &object));
			}
		}
	}

	pub fn iter_by_instance_class(
		self: &Store,
		class: &str,
	) -> std::collections::hash_set::Iter<Subject> {
		if let Some(inner) = self.classes.get(class) {
			inner.iter()
		} else {
			self.empty.iter()
		}
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
