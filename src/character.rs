use super::core::*;
use super::level::*;
// use super::player::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

pub fn can_move_to(store: &Store, loc: Point) -> bool {
	// level.is_valid(loc) && compatible_terrain(level, loc) && has_no_char(level, loc)
	is_level_loc(store, loc) && compatible_terrain(store, loc)
}

fn compatible_terrain(store: &Store, loc: Point) -> bool {
	match get_level_terrain(store, loc) {
		Terrain::DeepWater => {
			// let ch = level.character_components.get(&entity).unwrap();
			// ch.flags.has(CharacterFlags::Airborne) || ch.flags.has(CharacterFlags::Aquatic)
			false
		}
		Terrain::Ground => true,
		Terrain::ShallowWater => true,
		Terrain::Wall => false, // TODO: add support for status effects
	}
}

pub fn move_char_by(store: &mut Store, name: &Subject, dx: i32, dy: i32) -> Option<Duration> {
	assert!(dx != 0 || dy != 0);

	let old_loc = store.lookup_pt(name, Predicate::Loc).unwrap();

	let new_loc = Point {
		x: old_loc.x + dx,
		y: old_loc.y + dy,
	};
	if can_move_to(store, new_loc) {
		store.insert(name, Predicate::Loc, Object::Point(new_loc));
		if dx != 0 && dy != 0 {
			Some(Duration::from_secs(1.4 * 2.0))
		} else {
			Some(Duration::from_secs(2.0))
		}
	} else {
		None
	}
}

pub fn find_char_loc(store: &Store, rng: &mut SmallRng) -> Option<Point> {
	let size = get_level_size(store);
	let mut indexes: Vec<i32> = (0..size.width * size.height).collect();
	indexes.shuffle(rng);

	for i in &indexes {
		let x = i % size.width;
		let y = i / size.width;
		let loc = Point::new(x, y);
		let subject = cell(loc);
		if let Some(Terrain::Ground) = store.lookup_terrain(&subject, Predicate::Terrain) {
			if store.lookup_ref(&subject, Predicate::Character).is_none() {
				return Some(loc);
			}
		}
	}
	None
}
