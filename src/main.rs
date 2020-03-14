mod core;
mod level;
mod level_generator;
mod player;
mod terminal;

fn main() {
	let mut store = core::EventStore::new();
	let mut level = level::Level::new();
	let mut level_gen = level_generator::LevelGenerator::new();
	let mut player = player::Player::new();
	let mut terminal = terminal::Terminal::new();

	let mut queued = core::QueuedEvents::new();
	queued.push_back(core::Event::NewBranch);

	let mut running = true;
	while running {
		// Grab the next event,
		let event = queued.pop_front();
		println!("processing {:?}", event);

		// save it into the store (so that if there is a problem we can replay
		// the event that caused it),
		store.append(&event);

		// and give each service a chance to respond to the event.
		level.on_event(&event, &mut queued); // TODO: if signatures remain the same could use a vector of some trait
		level_gen.on_event(&event, &mut queued);
		player.on_event(&event, &mut queued, &level);
		running = terminal.on_event(&event, &mut queued, &level);
	}
}
