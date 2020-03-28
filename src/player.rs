use super::character::*;
use super::core::*;
use super::level::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlayerAction {
	DeltaEast,
	DeltaNorth,
	DeltaNorthEast,
	DeltaNorthWest,
	DeltaSouth,
	DeltaSouthEast,
	DeltaSouthWest,
	DeltaWest,
	Quit,
}

pub enum PlayerActionResult {
	Acted(Duration),

	/// Action doesn't apply to the player.
	Ignored,

	/// Player tried to do something but was unable to, e.g. moving into a wall.
	Error,
}

pub fn new_player(store: &mut Store) {
	let loc = Point::origin();
	store.insert(&PLAYER, Predicate::Loc, Object::Point(loc));
}

pub fn player_loc(store: &Store) -> Point {
	store.lookup_pt(&PLAYER, Predicate::Loc).unwrap()
}

pub fn on_player_event(
	store: &mut Store,
	rng: &mut SmallRng,
	event: &Event,
	pending: &mut PendingEvents,
) {
	match event {
		Event::NewLevel => {
			let loc = find_initial_loc(store, rng).unwrap();
			pending.push_back(Event::SetPlayer(loc));
		}
		Event::SetPlayer(loc) => {
			// TODO: should have an assert here (or maybe in Level) that loc is sane
			store.insert(&PLAYER, Predicate::Loc, Object::Point(*loc));
		}
		_ => (),
	}
}

pub fn on_player_action(store: &mut Store, action: PlayerAction) -> PlayerActionResult {
	match action {
		PlayerAction::DeltaEast => move_player_by(store, 1, 0),
		PlayerAction::DeltaNorth => move_player_by(store, 0, -1),
		PlayerAction::DeltaNorthEast => move_player_by(store, 1, -1),
		PlayerAction::DeltaNorthWest => move_player_by(store, -1, -1),
		PlayerAction::DeltaSouth => move_player_by(store, 0, 1),
		PlayerAction::DeltaSouthEast => move_player_by(store, 1, 1),
		PlayerAction::DeltaSouthWest => move_player_by(store, -1, 1),
		PlayerAction::DeltaWest => move_player_by(store, -1, 0),
		_ => PlayerActionResult::Ignored,
	}
}

fn move_player_by(store: &mut Store, dx: i32, dy: i32) -> PlayerActionResult {
	assert!(dx != 0 || dy != 0);

	let old_loc = store.lookup_pt(&PLAYER, Predicate::Loc).unwrap();

	let new_loc = Point {
		x: old_loc.x + dx,
		y: old_loc.y + dy,
	};
	if can_move_to(store, new_loc) {
		store.insert(&PLAYER, Predicate::Loc, Object::Point(new_loc));
		if dx != 0 && dy != 0 {
			PlayerActionResult::Acted(Duration::from_secs(1.4 * 2.0))
		} else {
			PlayerActionResult::Acted(Duration::from_secs(2.0))
		}
	} else {
		PlayerActionResult::Error // TODO: should we include a reason?
	}
}

fn find_initial_loc(store: &Store, rng: &mut SmallRng) -> Option<Point> {
	let size = get_level_size(store);
	let mut indexes: Vec<i32> = (0..size.width * size.height).collect();
	indexes.shuffle(rng);

	for i in &indexes {
		let x = i % size.width;
		let y = i / size.width;
		let loc = Point::new(x, y);
		let cell = get_level_terrain(store, loc);
		if let Terrain::Ground = cell {
			// if cell.character.is_none() && predicate(cell) {
			return Some(loc);
		}
	}
	None
}
