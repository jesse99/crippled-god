pub mod main_branch;

use super::core::*;

// LevelGenerator doesn't have any state but we use a struct anyway to keep it consistent with
// the other services.
pub struct LevelGenerator {}

impl LevelGenerator {
	pub fn new() -> LevelGenerator {
		LevelGenerator {}
	}

	pub fn on_event(&mut self, event: &Event, queued: &mut QueuedEvents) {
		if let Event::NewBranch = event {
			// TODO: probably want some sort of invariant check here
			// eg: that perimeter is some sort of permanent wall
			// and open areas exist
			// and maybe that all open areas are reachable
			main_branch::new(queued);
		}
	}
}
