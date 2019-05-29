
use super::character::CharacterFlags;
use super::entity::Entity;
use super::level::Level;
use super::location::Location;
use super::terrain::Terrain;

pub mod move_system {
	use super::*;

	/// Can be used to move arbitrary distances (e.g. teleport or blink).
	pub fn move_to(level: &mut Level, entity: Entity, loc: Location) {
		level.position_components.insert(entity, loc);
		debug!(level.logger, "moved"; "name" => entity, "new_loc" => loc);
	}

	pub fn can_move_to(level: &Level, entity: Entity, loc: Location) -> bool {
		valid_loc(level, loc) && compatible_terrain(level, entity, loc)
	}

	pub fn valid_loc(level: &Level, loc: Location) -> bool {
		loc.x >= 0
			&& loc.y >= 0
			&& loc.x < level.terrain.size().width
			&& loc.y < level.terrain.size().height
	}

	pub fn compatible_terrain(level: &Level, entity: Entity, loc: Location) -> bool {
		match level.terrain.get(loc) {
			Terrain::DeepWater => {
				let ch = level.character_components.get(&entity).unwrap();
				ch.flags.has(CharacterFlags::Airborne) || ch.flags.has(CharacterFlags::Aquatic)
			}
			Terrain::Ground => true,
			Terrain::ShallowWater => true,
			Terrain::Wall => false, // TODO: add support for status effects
		}
	}
}

pub mod player_system {
	use super::*;

	/// Called in response to the user pressing an arrow key. Several things can happen here including:
	/// 1) If current location + delta doesn't have an NPC and is a compatible terrain then move the
	/// player there.
	/// 2) If that location does have an NPC then attack it.
	/// 3) Manipulate an object, e.g. open or close a door.
	/// 4) Do nothing, e.g. when trying to move into a wall.
	pub fn delta_player_system(level: &mut Level, delta: Location) {
		assert!(
			delta.x >= -1
				&& delta.x <= 1 && delta.y >= -1
				&& delta.y <= 1 && delta != Location::zero(),
			"delta should be one square away {}",
			delta
		);

		let loc = *(level.position_components.get(&level.player).unwrap()) + delta;
		if move_system::can_move_to(level, level.player, loc) {
			move_system::move_to(level, level.player, loc);
		} else {
			let terrain = level.terrain.get(loc);
			debug!(level.logger, "player can't move"; "new_loc" => loc, "terrain" => terrain);
		}
	}
}
