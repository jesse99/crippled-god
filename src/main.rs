mod core;
mod level;
mod level_generator;

fn main() {
	let mut store = core::EventStore::new();
	let mut level = level::Level::new();
	let mut level_gen = level_generator::LevelGenerator::new();

	let mut queued = core::QueuedEvents::new();
	queued.push_back(core::Event::NewBranch);

	while !queued.is_empty() {
		// Grab the next event,
		let event = queued.pop_front();
		// println!("processing {:?}", event);

		// save it into the store (so that if there is a problem we can replay
		// the event that caused it),
		store.append(&event);

		// and give each service a chance to respond to the event.
		level.on_event(&event, &mut queued); // TODO: if signatures remain the same could use a vector of some trait
		level_gen.on_event(&event, &mut queued);
	}
}
