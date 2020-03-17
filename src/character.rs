use super::core::*;
use super::level::*;
// use super::player::*;

pub fn can_move_to(level: &Level, loc: Point) -> bool {
	// level.is_valid(loc) && compatible_terrain(level, loc) && has_no_char(level, loc)
	level.is_valid(loc) && compatible_terrain(level, loc)
}

fn compatible_terrain(level: &Level, loc: Point) -> bool {
	match level.get(loc) {
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
