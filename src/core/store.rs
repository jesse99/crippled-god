//! This module defines a Store struct which is used to manage a set of RDF style triplets.
//! The triplets contain a Subject, Predicate, and Object. For example,
//!    ("wolf-1", "description", "It has pointy teeth.")
//! The store encodes the entire game state and is operated upon by various components to
//! change state as the game is played and then to render the game. Note that there are
//! separate stores for each level.
use fnv::FnvHashMap;

/// This is used to identify an object within the game, eg an instance of an
/// NPC, the player, a location within the map, etc.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Subject(String);

impl Subject {
	/// Creates a unique subject, e.g. "player".
	pub fn new_unique(name: &str) -> Subject {
		Subject(name.to_string())
	}

	/// Creates an instance of a subject, e.g. "wolf".
	pub fn new_instance(store: &mut Store, name: &str) -> Subject {
		Subject(store.instance_name(name))
	}
}

/// Used to form a relation between a Subject and an Object.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Predicate {
	Height,
	Items,
	NPC,
	Terrain,
	Width,
}

/// The value associated with a Subject and relation.
pub enum Object {
	Int(i32),
	Ref(Subject),
	Refs(Vec<Subject>),
	Str(String),
	UInt(u32),
}

pub struct Store {
	count: u64,
	data: FnvHashMap<Subject, FnvHashMap<Predicate, Object>>,
}

// TODO:
// do we want a Time Object variant?
// should we have lookup functions that expect a particular Object variant?
impl Store {
	// TODO: may want to replace this with a function that loads from a trait
	pub fn new() -> Store {
		Store {
			count: 0,
			data: FnvHashMap::default(),
		}
	}

	pub fn insert(store: &mut Store, subject: Subject, predicate: Predicate, object: Object) {
		// TODO: May want to do some profiling to see:
		// 1) If the store methods are a bottle neck.
		// 2) If a HashMap<Subject, [({Predicate, Object})] would be better.
		// 3) If a flat [(Subject, Predicate, Object would be better)].
		let inner = store.data.entry(subject).or_default();
		inner.insert(predicate, object);
	}

	pub fn lookup<'a>(
		store: &'a Store,
		subject: &Subject,
		predicate: &Predicate,
	) -> Option<&'a Object> {
		if let Some(inner) = store.data.get(subject) {
			inner.get(predicate)
		} else {
			None
		}
	}

	fn instance_name(&mut self, base: &str) -> String {
		let name = format!("{}-{}", base, self.count);
		self.count += 1;
		name
	}
}
