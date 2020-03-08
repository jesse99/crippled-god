use super::*;

// Events which have executed.
pub struct EventStore {}

// Events which are pending execution.
pub struct QueuedEvents {
	events: Vec<Event>, // TODO: use a deque?
}

impl EventStore {
	pub fn new() -> EventStore {
		EventStore {}
	}

	pub fn append(&mut self, _event: &Event) {
		// TODO: persist it (probably want to flushh too)
	}
}

impl QueuedEvents {
	pub fn new() -> QueuedEvents {
		QueuedEvents { events: Vec::new() }
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
