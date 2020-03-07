use super::*;

// LevelGenerator doesn't have any state but we use a struct anyway to keep it consistent with
// the other services.
pub struct LevelGenerator {}

impl LevelGenerator {
	/// Levels start out empty and become populated as events occur.
	pub fn new() -> LevelGenerator {
		LevelGenerator {}
	}

	pub fn on_event(&mut self, events: &mut Events, level: &mut Level) {
		match events.last() {
			// TODO: probably want some sort of invariant check here
			// eg: that perimeter is some sort of permanent wall
			// and open areas exist
			// and maybe that all open areas are reachable
			Event::NewLevel => new_main(events, level),
			_ => (),
		}
	}
}

// Create a new level for the main branch.
fn new_main(events: &mut Events, level: &mut Level) {
	for x in 0..level.size().width {
		// North wall
		let loc = Point::new(x, 0);
		events.append(Event::SetTerrain(loc, Terrain::Wall));

		// South wall
		let loc = Point::new(x, level.size().height - 1);
		events.append(Event::SetTerrain(loc, Terrain::Wall));
	}

	for y in 1..(level.size().height - 1) {
		// West wall
		let loc = Point::new(0, y);
		events.append(Event::SetTerrain(loc, Terrain::Wall));

		// East wall
		let loc = Point::new(level.size().width - 1, y);
		events.append(Event::SetTerrain(loc, Terrain::Wall));
	}

	// Interior
	for x in 1..(level.size().width - 1) {
		for y in 1..(level.size().height - 1) {
			let loc = Point::new(x, y);
			events.append(Event::SetTerrain(loc, Terrain::Ground));
		}
	}
}
