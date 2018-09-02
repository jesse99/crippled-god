use super::pov::*;
use super::vec2::*;
use super::*;
use backend::terrain::MovementSpeed;
use rand;
// use rand::SeedableRng;
use std::collections::HashMap;
use std::fmt;

/// Set if the player or an NPC is within a Cell on the Level.
#[derive(Clone, Copy)]
pub enum Character {
	Player(Race),
	None,
}

/// Used to render a location within the Level.
#[derive(Clone)]
pub struct Cell {
	/// If visible is set then terrain and character will be up to date. Otherwise terrain will be blank if
	/// the user has never seen the Cell and whatever he saw last if he has seen the Cell.
	pub terrain: Terrain,
	pub character: Character,

	/// True if the player can see the square
	pub visible: bool,
}

/// Contains all the info for a level in the game.
pub struct Level {
	geography: Geography,
	player_loc: Location,
	cells: Vec2<Cell>,
}

impl Level {
	pub fn new(player: &Player, rng: &mut rand::XorShiftRng) -> Level {
		let geography = Geography::new();
		let cells = Vec2::new(geography.size(), Level::DEFAULT_CELL);
		let player_loc = geography
			.find_loc_with(rng, |t| player.race().speed(t) > 0.0)
			.expect("failed to find a location when new'ing the player");
		Level {
			geography,
			player_loc,
			cells,
		}
	}

	pub fn geography(&self) -> &Geography {
		&self.geography
	}

	pub fn player_loc(&self) -> Location {
		self.player_loc
	}

	pub fn move_player(&mut self, player: &Player, loc: Location) {
		assert!(player.can_move_to(self, loc));
		self.player_loc = loc;
	}

	/// screen_size is the number of Cells the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho that info may be outdated). It can also be arbitrarily small
	/// though in that case the user may not be able to see all the Cells the player can.
	/// Note that this is normally accessed through the Game method with the same name.
	pub fn get_cells(&mut self, player: &Player, screen_size: Size) -> Vec2<Cell> {
		self.toggle_cells(player);
		self.screen_cells(screen_size)
	}

	// ---- Private Items ---------------------------------------------------------------
	fn screen_cells(&self, screen_size: Size) -> Vec2<Cell> {
		let mut cells = Vec2::new(screen_size, Level::DEFAULT_CELL);
		let start_x = self.player_loc.x - screen_size.width / 2;
		let start_y = self.player_loc.y - screen_size.height / 2;
		for out_y in 0..screen_size.height {
			for out_x in 0..screen_size.width {
				let in_loc = Location::new(start_x + out_x, start_y + out_y);
				if in_loc.x >= 0
					&& in_loc.x < self.cells.size().width
					&& in_loc.y >= 0
					&& in_loc.y < self.cells.size().height
				{
					let cell = self.cells.get(in_loc);

					let out_loc = Location::new(out_x, out_y);
					cells.set(out_loc, cell.clone());
				}
			}
		}
		cells
	}

	const DEFAULT_CELL: Cell = Cell {
		terrain: Terrain::Blank,
		character: Character::None,
		visible: false,
	};

	fn toggle_cells(&mut self, player: &Player) {
		// The borrow checker won't allow us to grab a mutable reference to cells in one closure and
		// another reference in the second closure so we need to figure out what we need to do before
		// we call apply.
		let mut visible = HashMap::new(); // TODO: don't use a cryptograhic hasher
		{
			let visit = |loc| {
				let terrain = self.geography.at(loc);
				if self.player_loc == loc {
					visible.insert(loc, (terrain, Character::Player(player.race())));
				} else {
					visible.insert(loc, (terrain, Character::None));
				}
			};
			let blocks = |loc| {
				let terrain = self.geography.at(loc);
				terrain.blocks_los()
			};
			let radius = 10; // TODO: depends on race?
			visit_visible_cells(self.player_loc, self.cells.size(), radius, visit, blocks);
		}

		self.cells.apply(|loc, cell| match visible.get(&loc) {
			Some((terrain, ch)) => {
				cell.terrain = *terrain;
				cell.character = *ch;
				cell.visible = true;
			}
			None => cell.visible = false,
		})
	}
}

impl fmt::Debug for Cell {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.character {
			Character::Player(_) => write!(f, "@"),
			Character::None => write!(f, "{:?}", self.terrain),
		}
	}
}

impl fmt::Debug for Level {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\n")?;
		for y in 0..self.geography.size().height {
			for x in 0..self.geography.size().width {
				let loc = Location::new(x, y);
				if self.player_loc == loc {
					write!(f, "@")?;
				} else {
					write!(f, "{:?}", self.geography.at(loc))?;
				}
			}
			if y + 1 < self.geography.size().height {
				write!(f, "\n")?;
			}
		}
		write!(f, "")
	}
}
