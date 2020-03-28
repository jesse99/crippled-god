mod main_branch;

use super::core::*;

pub fn level_gen_ready_time(_store: &Store) -> Time {
	INFINITE_TIME
}

pub fn on_level_gen_event(_store: &mut Store, event: &Event, pending: &mut PendingEvents) {
	if let Event::NewBranch = event {
		// TODO: probably want some sort of invariant check here
		// eg: that perimeter is some sort of permanent wall
		// and open areas exist
		// and maybe that all open areas are reachable
		main_branch::new(pending);
		pending.push_back(Event::NewLevel);
	}
}
