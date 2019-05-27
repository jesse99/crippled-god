
use super::flags::Flags;
use super::location::Location;
use super::size::Size;
use super::terrain::Terrain;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum CharacterFlags {
	// Large,
	// Small,

	Airborne,
	Aquatic,
}

pub struct CharacterComponent {
	name: String,
	flags: Flags<CharacterFlags>,
}

impl CharacterComponent {
	pub fn new(name: &str, flags: Flags<CharacterFlags>) -> CharacterComponent {
		CharacterComponent {name: name.to_string(), flags}
	}
}
