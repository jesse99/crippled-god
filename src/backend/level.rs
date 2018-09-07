use super::pov::*;
use super::vec2::*;
use super::*;
use backend::terrain::MovementSpeed;
use rand;
// use rand::SeedableRng;
use fnv::FnvHashMap;
use std::fmt;

/// Set if the player or an NPC is within a Cell on the Level.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Character {
	Player(Race),
	None,
}

/// Used to render a location within the Level.
#[derive(Clone, Deserialize, Serialize)]
pub struct Cell {
	/// If visible is set then terrain and character will be up to date. Otherwise terrain will be blank if
	/// the user has never seen the Cell and whatever he saw last if he has seen the Cell.
	pub terrain: Terrain,
	pub character: Character,

	/// True if the player can see the square
	pub visible: bool,
}

/// Contains all the info for a level in the game.
#[derive(Deserialize, Serialize)]
pub struct Level {
	geography: Geography,
	cells: Vec2<Cell>,
}

impl Level {
	pub fn new() -> Level {
		let geography = Geography::new();
		let cells = Vec2::new(geography.size(), Level::DEFAULT_CELL);
		Level { geography, cells }
	}

	pub fn geography(&self) -> &Geography {
		&self.geography
	}

	/// screen_size is the number of Cells the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho that info may be outdated). It can also be arbitrarily small
	/// though in that case the user may not be able to see all the Cells the player can.
	/// Note that this is normally accessed through the Game method with the same name.
	pub fn get_cells(&mut self, player: &Player, screen_size: Size) -> Vec2<Cell> {
		self.toggle_cells(player);
		self.screen_cells(player, screen_size)
	}

	// ---- Private Items ---------------------------------------------------------------
	fn screen_cells(&self, player: &Player, screen_size: Size) -> Vec2<Cell> {
		let mut cells = Vec2::new(screen_size, Level::DEFAULT_CELL);
		let start_x = player.loc().x - screen_size.width / 2;
		let start_y = player.loc().y - screen_size.height / 2;
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
		let mut visible = FnvHashMap::default();
		{
			let visit = |loc| {
				let terrain = self.geography.at(loc);
				if player.loc() == loc {
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
			visit_visible_cells(player.loc(), self.cells.size(), radius, visit, blocks);
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
				write!(f, "{:?}", self.geography.at(loc))?;
			}
			if y + 1 < self.geography.size().height {
				write!(f, "\n")?;
			}
		}
		write!(f, "")
	}
}
