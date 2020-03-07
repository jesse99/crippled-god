use super::*;

pub struct Events {
	events: Vec<Event>,
}

impl Events {
	pub fn new() -> Events {
		Events { events: Vec::new() }
	}

	pub fn len(&self) -> usize {
		self.events.len()
	}

	pub fn append(&mut self, event: Event) {
		self.events.push(event);
	}

	pub fn last(&self) -> &Event {
		self.events
			.last()
			.expect("Should always have at least one event")
	}
}
