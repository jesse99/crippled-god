use super::core::*;
use super::level::*;
// use super::player::*;

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
