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
	store.insert(
		&PLAYER,
		Predicate::Ready,
		Object::Time(Time::from_secs(1.0)),
	);
}

pub fn player_loc(store: &Store) -> Point {
	store.lookup_pt(&PLAYER, Predicate::Loc).unwrap()
}

pub fn player_ready_time(store: &Store) -> Time {
	store.lookup_time(&PLAYER, Predicate::Ready).unwrap()
}

pub fn on_player_event(
	store: &mut Store,
	rng: &mut SmallRng,
	event: &Event,
	pending: &mut PendingEvents,
) {
	match event {
		Event::NewLevel => {
			let loc = find_char_loc(store, rng).unwrap();
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
	if let Some(duration) = move_char_by(store, &PLAYER, dx, dy) {
		PlayerActionResult::Acted(duration)
	} else {
		PlayerActionResult::Error // TODO: should we include a reason?
	}
}
