use super::*;

// Events which have executed.
pub struct ExecutedEvents {}

// [`Event`]s which are pending execution.
pub struct PendingEvents {
	events: Vec<Event>, // TODO: use a deque?
}

impl ExecutedEvents {
	pub fn new() -> ExecutedEvents {
		ExecutedEvents {}
	}

	pub fn append(&mut self, _event: &Event) {
		// TODO: persist it (probably want to flush too)
	}
}

impl PendingEvents {
	pub fn new() -> PendingEvents {
		PendingEvents { events: Vec::new() }
	}

	pub fn is_empty(&self) -> bool {
		self.events.is_empty()
	}

	pub fn push_back(&mut self, event: Event) {
		self.events.push(event);
	}

	pub fn pop_front(&mut self) -> Event {
		self.events.remove(0)
	}
}
