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
	let event = Event::NewGame;
	let loc = Point::origin();
	store.insert(&event, &PLAYER, Predicate::Loc, Object::Point(loc));
	store.insert(
		&event,
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
			store.insert(event, &PLAYER, Predicate::Loc, Object::Point(*loc));
		}
		_ => (),
	}
}

pub fn on_player_action(
	store: &Store,
	pending: &mut PendingEvents,
	action: PlayerAction,
) -> PlayerActionResult {
	match action {
		PlayerAction::DeltaEast => move_player_by(store, pending, 1, 0),
		PlayerAction::DeltaNorth => move_player_by(store, pending, 0, -1),
		PlayerAction::DeltaNorthEast => move_player_by(store, pending, 1, -1),
		PlayerAction::DeltaNorthWest => move_player_by(store, pending, -1, -1),
		PlayerAction::DeltaSouth => move_player_by(store, pending, 0, 1),
		PlayerAction::DeltaSouthEast => move_player_by(store, pending, 1, 1),
		PlayerAction::DeltaSouthWest => move_player_by(store, pending, -1, 1),
		PlayerAction::DeltaWest => move_player_by(store, pending, -1, 0),
		_ => PlayerActionResult::Ignored,
	}
}

fn move_player_by(
	store: &Store,
	pending: &mut PendingEvents,
	dx: i32,
	dy: i32,
) -> PlayerActionResult {
	if let Some((duration, new_loc)) = move_char_by(store, &PLAYER, dx, dy) {
		pending.push_back(Event::SetPlayer(new_loc));
		PlayerActionResult::Acted(duration)
	} else {
		PlayerActionResult::Error // TODO: should we include a reason?
	}
}
