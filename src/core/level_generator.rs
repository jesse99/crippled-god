use super::*;

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
			new_main(queued);
		}
	}
}

// Create a new level for the main branch.
fn new_main(queued: &mut QueuedEvents) {
	let size = Size::new(20, 10);
	queued.push_back(Event::ResetLevel(
		"Level 1".to_string(),
		size,
		Terrain::Wall,
	));

	for x in 0..size.width {
		// North wall
		let loc = Point::new(x, 0);
		queued.push_back(Event::SetTerrain(loc, Terrain::Wall));

		// South wall
		let loc = Point::new(x, size.height - 1);
		queued.push_back(Event::SetTerrain(loc, Terrain::Wall));
	}

	for y in 1..(size.height - 1) {
		// West wall
		let loc = Point::new(0, y);
		queued.push_back(Event::SetTerrain(loc, Terrain::Wall));

		// East wall
		let loc = Point::new(size.width - 1, y);
		queued.push_back(Event::SetTerrain(loc, Terrain::Wall));
	}

	// Interior
	for x in 1..(size.width - 1) {
		for y in 1..(size.height - 1) {
			let loc = Point::new(x, y);
			queued.push_back(Event::SetTerrain(loc, Terrain::Ground));
		}
	}

	queued.push_back(Event::NewLevel);
}
