use super::geography::Geography;
use super::location::Location;
use super::player::*;
use super::size::Size;
use super::terrain::Terrain;
use super::vec2::Vec2;
use rand;
use rand::SeedableRng;
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
	pub visible: bool, // i.e. can the player see this cell
}

/// Contains all the info for a level in the game.
pub struct Level {
	pub geography: Geography,
	pub player: Player,
	rng: rand::XorShiftRng,
	cells: Vec2<Cell>,
}

impl Level {
	pub fn new(seed: usize) -> Level {
		let seed = [
			((seed >> 24) & 0xFF) as u8,
			((seed >> 16) & 0xFF) as u8,
			((seed >> 8) & 0xFF) as u8,
			(seed & 0xFF) as u8,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
			0,
		];
		let mut rng = rand::XorShiftRng::from_seed(seed);

		let geography = Geography::new();
		let player = Player::new(Race::Human, &geography, &mut rng);
		let cells = Vec2::new(geography.size(), Level::DEFAULT_CELL);
		Level {
			rng,
			geography,
			player,
			cells,
		}
	}

	pub fn move_player(&mut self, loc: Location) {
		assert!(self.player.can_move_to(self, loc));
		self.player.loc = loc;
	}

	// TODO:
	// wire in a real visibibility check (probably should have a unit test for this)
	// geography should use Vec2

	/// screen_size is the number of Cells the renderer wants to render. This can be
	/// arbitrarily large in which case the user will be able to see more of what he
	/// saw earlier (tho that info may be outdated). It can also be arbitrarily small
	/// though in that case the user may not be able to see all the Cells the player can.
	pub fn get_cells(&mut self, screen_size: Size) -> Vec2<Cell> {
		self.toggle_cells();
		self.screen_cells(screen_size)
	}

	// ---- Private Items ---------------------------------------------------------------
	fn screen_cells(&self, screen_size: Size) -> Vec2<Cell> {
		let mut cells = Vec2::new(screen_size, Level::DEFAULT_CELL);
		let start_x = self.player.loc.x - screen_size.width / 2;
		let start_y = self.player.loc.y - screen_size.height / 2;
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

	// TODO: try implementing http://www.roguebasin.com/index.php?title=Permissive_Field_of_View_in_Python
	fn can_see(&self, player: &Player, loc: Location) -> bool {
		player.loc.distance(loc) < 10.0
	}

	fn toggle_cells(&mut self) {
		let player = self.player.clone();

		// The borrow checker won't allow us to grab references to self inside the apply
		// loop below so we need to figure out what we need to do before we call apply.
		let mut visible = HashMap::new(); // TODO: don't use a cryptograhic hasher
		for (loc, _) in self.cells.iter() {
			if self.can_see(&player, loc) {
				let terrain = self.geography.at(loc);
				if self.player.is_at(loc) {
					visible.insert(loc, (terrain, Character::Player(player.race)));
				} else {
					visible.insert(loc, (terrain, Character::None));
				}
			}
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
				if self.player.is_at(loc) {
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
