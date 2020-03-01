//! This module defines a Store struct which is used to manage a set of Key/Value pairs.
//! This encodes the entire game state and is operated upon by various components to change
//! state as the game is played and then to render the game. Note that there are separate
//! stores for each level.
//!
//! These pairs are inspired by RDF subject, predicate, object triplets but binding the
//! value directly to the predicate makes it much easier to operate on the store because
//! the value type is always known.
use super::point;
use super::terrain;

/// This is used to identify an object within the game, eg an instance of an
/// NPC, the player, a location within the map, etc.
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

/// Associates a Subject with a predicate/value pair.
pub enum Relation {
	/// A name to be used by UIs.
	DisplayName(String),

	// Ground, DeepWater, Wall, etc.
	Feature(terrain::Terrain),

	/// Location within the map where (0, 0) is topLeft.
	Location(point::Point), // TODO: can we validate these when they are added to the store?
}

pub struct Store {
	count: u64,
	// TODO: need a hash from subject to [relation]
}

impl Store {
	fn instance_name(&mut self, base: &str) -> String {
		let name = format!("{}-{}", base, self.count);
		self.count += 1;
		name
	}

	// TODO:
	// insert method (or add/replace?)
	// find/iterate
}
