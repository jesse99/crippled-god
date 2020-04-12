use super::character::*;
use super::core::*;
use super::level::*;
use rand::rngs::SmallRng;
// use rand::seq::SliceRandom;

// TODO: need to do this via an event
// pub fn new_npc(store: &mut Store, prefix: &str, rng: &mut SmallRng, current_time: Time) {
// 	if let Some(loc) = find_char_loc(store, rng) {
// 		let name = Subject::new_instance(store, "npc", prefix);
// 		store.insert(&name, Predicate::Loc, Object::Point(loc));
// 		store.insert(
// 			&name,
// 			Predicate::Ready,
// 			Object::Time(current_time + Duration::from_secs(1.1)),
// 		);
// 	}
// 	// TODO: else log?
// }

pub fn npc_ready_time(store: &Store) -> Time {
	let mut time = INFINITE_TIME;

	for name in store.iter_by_instance_class("npc") {
		let candidate = store.lookup_time(name, Predicate::Ready).unwrap();
		if candidate < time {
			time = candidate;
		}
	}

	time
}

pub fn on_npc_event(
	store: &mut Store,
	rng: &mut SmallRng,
	event: &Event,
	pending: &mut PendingEvents,
) {
	match event {
		Event::AdvanceTime(time) => {
			let names: Vec<Subject> = store
				.iter_by_instance_class("npc")
				.filter(|name| {
					let ready = store.lookup_time(name, Predicate::Ready).unwrap();
					assert!(*time >= ready);
					*time == ready
				})
				.cloned()
				.collect();
			for name in names.iter() {
				do_skittish(store, rng, pending, name);
			}
		}
		_ => (),
	}
}

fn do_skittish(store: &mut Store, rng: &mut SmallRng, pending: &mut PendingEvents, name: &Subject) {
}

fn move_npc_by(store: &mut Store, name: &Subject, dx: i32, dy: i32) -> Option<Duration> {
	if let Some((duration, new_loc)) = move_char_by(store, name, dx, dy) {
		// pending.push_back(Event::SetNPC(new_loc)); // TODO
		Some(duration)
	} else {
		None
	}
}
