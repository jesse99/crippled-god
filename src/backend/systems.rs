
use super::character::CharacterFlags;
use super::entity::Entity;
use super::level::Level;
use super::location::Location;
use super::terrain::Terrain;

pub mod move_system {
	use super::*;

	/// Can be used to move arbitrary distances (e.g. teleport or blink).
	pub fn move_by(level: &mut Level, entity: Entity, delta: Location) {
		let loc = level.position_components.get_mut(&entity).unwrap();
		*loc += delta;
	}

	pub fn can_move_by(level: &Level, entity: Entity, delta: Location) -> bool {
		if let Some(&loc) = level.position_components.get(&entity) {
			let loc = loc + delta;
			valid_loc(level, loc) && compatible_terrain(level, entity, loc)
		} else {
			false
		}
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
	fn delta_player_system(level: &mut Level, delta: Location) {
		assert!(
			delta.x >= -1 && delta.x <= 1,
			"delta should be one square away {}",
			delta
		);
		assert!(
			delta.y >= -1 && delta.y <= 1,
			"delta should be one square away {}",
			delta
		);
		assert!(
			delta != Location::zero(),
			"delta should be one square away {}",
			delta
		);

		if move_system::can_move_by(level, level.player, delta) {
			move_system::move_by(level, level.player, delta);
		}
	}
}
