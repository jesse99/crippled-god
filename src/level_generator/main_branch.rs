use super::super::core::*;

// Create a new level for the main branch.
pub fn new(pending: &mut PendingEvents) {
	let size = Size::new(100, 50);
	pending.push_back(Event::ResetLevel(
		"Level 1".to_string(),
		size,
		Terrain::Wall,
	));

	// Add walls around the outside
	for x in 0..size.width {
		set(pending, x, 0, Terrain::Wall);
		set(pending, x, size.height - 1, Terrain::Wall);
	}
	for y in 0..size.height {
		set(pending, 0, y, Terrain::Wall);
		set(pending, size.width - 1, y, Terrain::Wall);
	}

	// Interior
	for x in 1..(size.width - 1) {
		for y in 1..(size.height - 1) {
			let loc = Point::new(x, y);
			pending.push_back(Event::SetTerrain(loc, Terrain::Ground));
		}
	}

	// Add a little lake in the middle.
	let x = size.width / 2;
	let y = size.height / 2 - 1;
	set(pending, x, y, Terrain::ShallowWater);
	set(pending, x - 1, y + 1, Terrain::DeepWater);
	set(pending, x, y + 1, Terrain::DeepWater);
	set(pending, x + 1, y + 1, Terrain::ShallowWater);
	set(pending, x, y + 2, Terrain::ShallowWater);

	// Add a short wall.
	let y = 8;
	set(pending, x + 2, y, Terrain::Wall);
	set(pending, x + 1, y, Terrain::Wall);
	set(pending, x, y, Terrain::Wall);
	set(pending, x - 1, y, Terrain::Wall);
	set(pending, x - 2, y, Terrain::Wall);
}

fn set(pending: &mut PendingEvents, x: i32, y: i32, terrain: Terrain) {
	let loc = Point::new(x, y);
	pending.push_back(Event::SetTerrain(loc, terrain));
}
