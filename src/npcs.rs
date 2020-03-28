use super::core::*;
use super::npc::*;
// use super::level::*;
// use super::player::*;

pub struct NPCs {}

impl NPCs {
	pub fn get_npc(&self, loc: Point) -> Option<Box<dyn NPC>> {
		None
	}

	/// Returns the [`NPC`] with the smallest ready time.
	pub fn next_ready(&self) -> Option<Box<dyn NPC>> {
		None
	}
}
