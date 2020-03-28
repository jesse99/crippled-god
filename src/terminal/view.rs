use super::super::core::*;
// use super::super::level::*;
// use super::super::player::*;
use super::color;
use termion;

/// Visual representation of terrain, items, and characters on a position within the map.
pub struct View {
	pub symbol: char,
	pub fg: termion::color::AnsiValue,
	pub bg: termion::color::AnsiValue,
	// TODO: might want to add support for styles, see https://docs.rs/termion/1.5.1/termion/style/index.html
}

impl View {
	pub fn new(store: &Store, cell: &Subject) -> View {
		let seen_terrain = store.lookup_terrain(&cell, Predicate::LastSeenTerrain);
		let seen_char = store.lookup_ref(&cell, Predicate::LastSeenChar);
		if store.lookup_bool(&cell, Predicate::Visible).unwrap() {
			let bg = color::to_termion(if let Some(terrain) = seen_terrain {
				terrain.back_color()
			} else {
				color::Color::Black
			});
			if let Some(ch) = seen_char {
				// let symbol = game.get_species(entity).visible_symbol();
				let symbol = '@';
				let fg = if ch == *PLAYER {
					color::to_termion(color::Color::White)
				} else {
					color::to_termion(color::Color::Red)
				};
				View { symbol, fg, bg }
			} else if let Some(terrain) = seen_terrain {
				let fg = color::to_termion(terrain.fore_color());
				let symbol = terrain.visible_symbol();
				View { symbol, fg, bg }
			} else {
				let fg = color::to_termion(color::Color::Black);
				let symbol = '?';
				View { symbol, fg, bg }
			}
		} else {
			let bg = color::to_termion(color::Color::LightGrey);
			let fg = color::to_termion(color::Color::DarkGray);
			let symbol = if seen_char.is_some() {
				'@'
			} else if let Some(terrain) = seen_terrain {
				terrain.hidden_symbol()
			} else {
				' '
			};
			View { symbol, fg, bg }
		}
	}
}

// --- Private Items ----------------------------------------------------------
trait ToBackColor {
	fn back_color(&self) -> color::Color;
}

trait ToForeColor {
	fn fore_color(&self) -> color::Color;
}

trait VisibleSymbol {
	fn visible_symbol(&self) -> char;
}

trait HiddenSymbol {
	fn hidden_symbol(&self) -> char;
}

impl ToBackColor for Terrain {
	fn back_color(&self) -> color::Color {
		match self {
			Terrain::DeepWater => color::Color::LightBlue,
			Terrain::Ground => color::Color::Black,
			Terrain::Wall => color::Color::Black,
			Terrain::ShallowWater => color::Color::LightBlue,
		}
	}
}

impl ToForeColor for Terrain {
	fn fore_color(&self) -> color::Color {
		match self {
			Terrain::DeepWater => color::Color::Blue,
			Terrain::Ground => color::Color::LightSlateGray,
			Terrain::Wall => color::Color::Chocolate,
			Terrain::ShallowWater => color::Color::Blue,
		}
	}
}

// impl ToForeColor for CharName {
// 	fn fore_color(&self) -> color::Color {
// 		match self {
// 			CharName::Ay => color::Color::BurlyWood,
// 			CharName::Bhederin => color::Color::Chocolate,
// 			CharName::Human => color::Color::White,
// 		}
// 	}
// }

impl VisibleSymbol for Terrain {
	fn visible_symbol(&self) -> char {
		match self {
			Terrain::DeepWater => 'w',
			Terrain::Ground => ' ',
			Terrain::Wall => '#',
			Terrain::ShallowWater => '~',
		}
	}
}

// impl VisibleSymbol for Species {
// 	fn visible_symbol(&self) -> char {
// 		match self {
// 			Species::Ay => 'a',
// 			Species::Bhederin => 'b',
// 			Species::Human => '@',
// 		}
// 	}
// }

impl HiddenSymbol for Terrain {
	fn hidden_symbol(&self) -> char {
		match self {
			Terrain::DeepWater => self.visible_symbol(),
			Terrain::Ground => ' ',
			Terrain::Wall => self.visible_symbol(),
			Terrain::ShallowWater => self.visible_symbol(),
		}
	}
}

// impl VisibleSymbol for CharName {
// 	fn visible_symbol(&self) -> char {
// 		match self {
// 			CharName::Ay => 'a',
// 			CharName::Bhederin => 'b',
// 			CharName::Human => 'h',
// 		}
// 	}
// }
