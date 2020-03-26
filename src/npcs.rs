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
// TODO: Box cannot be copied, could use Rc instead of a Box...
// or use a reference to the box?
