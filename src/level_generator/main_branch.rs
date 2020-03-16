use super::super::core::*;

// Create a new level for the main branch.
pub fn new(queued: &mut QueuedEvents) {
	let size = Size::new(100, 50);
	queued.push_back(Event::ResetLevel(
		"Level 1".to_string(),
		size,
		Terrain::Wall,
	));

	// Add walls around the outside
	for x in 0..size.width {
		set(queued, x, 0, Terrain::Wall);
		set(queued, x, size.height - 1, Terrain::Wall);
	}
	for y in 0..size.height {
		set(queued, 0, y, Terrain::Wall);
		set(queued, size.width - 1, y, Terrain::Wall);
	}

	// Interior
	for x in 1..(size.width - 1) {
		for y in 1..(size.height - 1) {
			let loc = Point::new(x, y);
			queued.push_back(Event::SetTerrain(loc, Terrain::Ground));
		}
	}

	// Add a little lake in the middle.
	let x = size.width / 2;
	let y = size.height / 2 - 1;
	set(queued, x, y, Terrain::ShallowWater);
	set(queued, x - 1, y + 1, Terrain::DeepWater);
	set(queued, x, y + 1, Terrain::DeepWater);
	set(queued, x + 1, y + 1, Terrain::ShallowWater);
	set(queued, x, y + 2, Terrain::ShallowWater);

	// Add a short wall.
	let y = 8;
	set(queued, x + 2, y, Terrain::Wall);
	set(queued, x + 1, y, Terrain::Wall);
	set(queued, x, y, Terrain::Wall);
	set(queued, x - 1, y, Terrain::Wall);
	set(queued, x - 2, y, Terrain::Wall);
}

fn set(queued: &mut QueuedEvents, x: i32, y: i32, terrain: Terrain) {
	let loc = Point::new(x, y);
	queued.push_back(Event::SetTerrain(loc, terrain));
}
