
use super::super::Game;
use super::*;

fn move_duration(game: &Game, entity: Entity, loc: Location, delta: Location) -> Duration {
	let c = game.level.character_components.get(&entity).unwrap();
	let terrain = game.level.cells.get(loc + delta).terrain;
	let duration = c.species.move_duration(terrain);
	if delta.x != 0 && delta.y != 0 {
		duration.percent(1.414)
	} else {
		duration
	}
}

pub mod ai_system {
	use super::*;

	/// This is where entities other than the player figure out what they should do. A Some(duration)
	/// is returned based whatever they decided to do. None is returned if the entity poofed (e.g. 
	/// it died in combat or was some sort of transient item).
	pub fn act(game: &mut Game, entity: Entity) -> Option<Duration> {
		if let Some(c) = game.level.character_components.get(&entity) {
			match c.species {
				Species::Ay => aggressive(game, entity),
				Species::Bhederin => passive(game, entity),
				Species::Human => passive(game, entity),
			}
		} else {
			panic!("ai for non-character entities isn't supported yet")
		}
	}

	fn aggressive(game: &mut Game, entity: Entity) -> Option<Duration> {
		let ploc = game.level.player_loc();
		let left_is_better = |lhs, rhs| {
			let d1 = ploc.distance(lhs);
			let d2 = ploc.distance(rhs);
			d1 > 0.0 && d1 < d2
		};
		move_relative_to_player(game, entity, left_is_better)
	}

	fn passive(game: &mut Game, entity: Entity) -> Option<Duration> {
		let ploc = game.level.player_loc();
		let left_is_better = |lhs, rhs| {
			let d1 = ploc.distance(lhs);
			let d2 = ploc.distance(rhs);
			d1 > d2
		};
		move_relative_to_player(game, entity, left_is_better)
	}

	fn move_relative_to_player<F: Fn(Location, Location) -> bool>(game: &mut Game, entity: Entity, left_is_better: F) -> Option<Duration> {
		let mut delta = Location::zero();
		let loc = game.level.entity_loc(entity).expect(&format!("no position for {:?}", entity));
		if game.level.is_visible(loc, game.level.player_loc()) {
			let mut deltas = vec![
				Location::new(-1, -1),
				Location::new(-1, 0),
				Location::new(-1, 1),
				Location::new(0, -1),
				Location::new(0, 0),
				Location::new(0, 1),
				Location::new(1, -1),
				Location::new(1, 0),
				Location::new(1, 1),
			];

			game.level.rng.shuffle(&mut deltas);
			for candidate in deltas {
				if move_system::can_move_to(&game.level, entity, loc + candidate)
					&& left_is_better(loc + candidate, loc + delta)
				{
					delta = candidate;
				}
			}
		}
		if delta != Location::zero() {
			move_system::move_to(game, entity, loc + delta);
			Some(move_duration(game, entity, loc, delta))
		} else {
			Some(NO_OP_DURATION)
		}
	}
}

pub mod move_system {
	use super::*;

	/// Can be used to move arbitrary distances (e.g. teleport or blink).
	pub fn move_to(game: &mut Game, entity: Entity, loc: Location) {
		if let Some(old_loc) = game.level.position_components.insert(entity, loc) {
			game.level.cells.get_mut(old_loc).character = None;
		}
		game.level.cells.get_mut(loc).character = Some(entity);
		// debug!(game.level.logger, "moved"; "name" => entity, "new_loc" => loc);
	}

	pub fn can_move_to(level: &Level, entity: Entity, loc: Location) -> bool {
		valid_loc(level, loc) && compatible_terrain(level, entity, loc)
	}

	pub fn valid_loc(level: &Level, loc: Location) -> bool {
		loc.x >= 0
			&& loc.y >= 0
			&& loc.x < level.cells.size().width
			&& loc.y < level.cells.size().height
	}

	pub fn compatible_terrain(level: &Level, entity: Entity, loc: Location) -> bool {
		match level.cells.get(loc).terrain {
			Terrain::Blank => panic!("Blank should only be used for rendering"),
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

	/// Called in response to the user pressing an arrow or directional key. Several things can
	/// happen here including:
	/// 1) If current location + delta doesn't have an NPC and is a compatible terrain then move the
	/// player there.
	/// 2) If that location does have an NPC then attack it.
	/// 3) Manipulate an object, e.g. open or close a door.
	/// 4) Do nothing, e.g. when trying to move into a wall.
	pub fn delta_player_system(game: &mut Game, delta: Location) -> Duration {
		assert!(
			delta.x >= -1
				&& delta.x <= 1 && delta.y >= -1
				&& delta.y <= 1 && delta != Location::zero(),
			"delta should be one square away {}",
			delta
		);

		let loc = game.level.player_loc() + delta;
		let terrain = game.level.cells.get(loc).terrain;
		if move_system::can_move_to(&game.level, game.level.player, loc) {
			move_system::move_to(game, game.level.player, loc);
		}
		if let Some(message) = terrain.message_for(game, game.level.player) {
			game.add_message(message);
		}
		move_duration(game, game.level.player, loc, delta)
	}
}
