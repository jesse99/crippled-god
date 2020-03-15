use super::super::core::*;

// Create a new level for the main branch.
pub fn new(queued: &mut QueuedEvents) {
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

	queued.push_back(Event::SetTerrain(Point::new(4, 5), Terrain::ShallowWater));
	queued.push_back(Event::SetTerrain(Point::new(5, 5), Terrain::DeepWater));
	queued.push_back(Event::SetTerrain(Point::new(6, 5), Terrain::ShallowWater));
}
