
use super::entity::Entity;
use super::level::Level;
use super::location::Location;

/// Unlike delta_player_system this can be used to move multiple squares, e.g. something like a blink.
fn move_system(level: &mut Level, entity: Entity, delta: Location) {
	let loc = level.position_components.get_mut(&entity).unwrap();
	*loc += delta;
}

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

	move_system(level, level.player, delta);
}
