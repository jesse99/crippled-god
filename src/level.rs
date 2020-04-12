use super::core::*;
use super::player::*;
use fnv::FnvHashMap;

pub fn new_level(store: &mut Store) {
	store.insert(
		&LEVEL,
		Predicate::Name,
		Object::Str("uninitialized".to_string()),
	);
}

pub fn get_level_size(store: &Store) -> Size {
	store
		.lookup_size(&LEVEL, Predicate::Size)
		.unwrap_or_else(|| panic!("Couldn't find size for {:?}", *LEVEL))
}

pub fn is_level_loc(store: &Store, loc: Point) -> bool {
	let size = get_level_size(store);
	loc.x >= 0 && loc.x < size.width && loc.y >= 0 && loc.y < size.height
}

pub fn get_level_terrain(store: &Store, loc: Point) -> Terrain {
	let subject = cell(loc);
	store
		.lookup_terrain(&subject, Predicate::Terrain)
		.unwrap_or_else(|| panic!("Couldn't find terrain for {:?}", subject))
}

pub fn on_level_event(store: &mut Store, event: &Event, _pending: &mut PendingEvents) {
	match event {
		Event::ResetLevel(name, size, terrain) => {
			store.insert(event, &LEVEL, Predicate::Name, Object::Str(name.clone()));
			store.insert(event, &LEVEL, Predicate::Size, Object::Size(*size));

			let subject = Subject::new_unique("dummy-cell"); // TODO: should probaby handle this with a NewGame event
			store.insert(event, &subject, Predicate::Visible, Object::Bool(false));

			for y in 0..size.height {
				for x in 0..size.width {
					let subject = cell(Point::new(x, y));
					store.insert(
						event,
						&subject,
						Predicate::Terrain,
						Object::Terrain(*terrain),
					);
					store.insert(event, &subject, Predicate::Visible, Object::Bool(false));
				}
			}
		}
		Event::SetTerrain(loc, terrain) => {
			let subject = cell(*loc);
			store.insert(
				event,
				&subject,
				Predicate::Terrain,
				Object::Terrain(*terrain),
			);
		}
		_ => (),
	}
}

/// Returns a vector of locations (in screen coordinates) and a subject for the associated
/// cell.
///
/// screen_size is the number of tiles the renderer wants to render. This can be
/// arbitrarily large in which case the user will be able to see more of what he
/// saw earlier (tho if it is not within the player's LOS that info may be outdated).
/// It can also be arbitrarily small though in that case the user may not be able
/// to see all the tiles that the player should be able to.
pub fn get_last_seen(store: &mut Store, screen_size: Size) -> Vec<(Point, Subject)> {
	update_tiles(store);
	screen_tiles(store, screen_size)
	//self.invariant();		// TODO: probably want something like this somewhere
}

// Updates the tiles that are within the player's LOS.
fn update_tiles(store: &mut Store) {
	// The borrow checker won't allow us to grab a mutable reference to tiles in one closure and
	// another reference in the second closure so we'll figure out which cells are visible and
	// then update tiles.
	let size = get_level_size(store);
	let player_loc = player_loc(store);
	let mut visible = FnvHashMap::default();
	let mut pov = POV {
		start: player_loc,
		size,
		radius: 10, // TODO: depends on race?
		visible_tile: |loc| {
			let terrain = get_level_terrain(store, loc);
			visible.insert(loc, terrain);
		},
		blocks_los: |loc| {
			let terrain = get_level_terrain(store, loc);
			matches!(terrain, Terrain::Wall) // TODO: do something better here
		},
	};
	pov.visit();

	for y in 0..size.height {
		for x in 0..size.width {
			let loc = Point::new(x, y);
			let subject = cell(loc);
			if let Some(terrain) = visible.get(&loc) {
				if loc == player_loc {
					store.insert(
						&subject,
						Predicate::LastSeenChar,
						Object::Ref(PLAYER.clone()),
					);
				} else {
					store.remove(&subject, Predicate::LastSeenChar);
				};
				store.insert(
					&subject,
					Predicate::LastSeenTerrain,
					Object::Terrain(*terrain),
				);
				store.insert(&subject, Predicate::Visible, Object::Bool(true));
			} else {
				// leave the other state as it was when it was last within the player's LOS
				store.insert(&subject, Predicate::Visible, Object::Bool(false));
			}
		}
	}
}

// Returns the subset of tiles that are rendered on the screen.
fn screen_tiles(store: &Store, screen_size: Size) -> Vec<(Point, Subject)> {
	let mut tiles = Vec::new();
	let player_loc = player_loc(store);
	let start_x = player_loc.x - screen_size.width / 2;
	let start_y = player_loc.y - screen_size.height / 2;

	let size = get_level_size(store);
	for screen_y in 0..screen_size.height {
		for screen_x in 0..screen_size.width {
			let level_loc = Point::new(start_x + screen_x, start_y + screen_y);
			let screen_loc = Point::new(screen_x, screen_y);
			if level_loc.x >= 0
				&& level_loc.x < size.width
				&& level_loc.y >= 0
				&& level_loc.y < size.height
			{
				let subject = cell(level_loc);
				tiles.push((screen_loc, subject))
			} else {
				let subject = Subject::new_unique("dummy-cell");
				tiles.push((screen_loc, subject))
			}
		}
	}
	tiles
}
