use super::backend::{self, Game, Species, Tile};
use super::colors;
use termion;

/// Visual representation of terrain, items, and characters on a position within the map.
pub struct View {
	pub symbol: char,
	pub fg: termion::color::AnsiValue,
	pub bg: termion::color::AnsiValue,
	// TODO: might want to add support for styles, see https://docs.rs/termion/1.5.1/termion/style/index.html
}

impl View {
	pub fn new(game: &Game, tile: &Tile) -> View {
		if tile.visible {
			let bg = colors::to_termion(if let Some(terrain) = tile.terrain {terrain.back_color()} else {colors::Color::Black});
			if let Some(entity) = tile.character {
				let symbol = game.get_species(entity).visible_symbol();
				let fg = if game.is_player(entity) {
					colors::to_termion(colors::Color::White)
				} else {
					colors::to_termion(colors::Color::Red)
				};
				View { symbol, fg, bg }
			} else if let Some(terrain) = tile.terrain {
				let fg = colors::to_termion(terrain.fore_color());
				let symbol = terrain.visible_symbol();
				View { symbol, fg, bg }
			} else {
				let fg = colors::to_termion(colors::Color::Black);
				let symbol = '?';
				View { symbol, fg, bg }
			}
		} else {
			let bg = colors::to_termion(colors::Color::LightGrey);
			let fg = colors::to_termion(colors::Color::DarkGray);
			let symbol = if let Some(entity) = tile.character {
				game.get_species(entity).visible_symbol()
			} else if let Some(terrain) = tile.terrain {
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
	fn back_color(&self) -> colors::Color;
}

trait ToForeColor {
	fn fore_color(&self) -> colors::Color;
}

trait VisibleSymbol {
	fn visible_symbol(&self) -> char;
}

trait HiddenSymbol {
	fn hidden_symbol(&self) -> char;
}

impl ToBackColor for backend::Terrain {
	fn back_color(&self) -> colors::Color {
		match self {
			backend::Terrain::DeepWater => colors::Color::LightBlue,
			backend::Terrain::Ground => colors::Color::Black,
			backend::Terrain::Wall => colors::Color::Black,
			backend::Terrain::ShallowWater => colors::Color::LightBlue,
		}
	}
}

impl ToForeColor for backend::Terrain {
	fn fore_color(&self) -> colors::Color {
		match self {
			backend::Terrain::DeepWater => colors::Color::Blue,
			backend::Terrain::Ground => colors::Color::LightSlateGray,
			backend::Terrain::Wall => colors::Color::Chocolate,
			backend::Terrain::ShallowWater => colors::Color::Blue,
		}
	}
}

// impl ToForeColor for backend::CharName {
// 	fn fore_color(&self) -> colors::Color {
// 		match self {
// 			backend::CharName::Ay => colors::Color::BurlyWood,
// 			backend::CharName::Bhederin => colors::Color::Chocolate,
// 			backend::CharName::Human => colors::Color::White,
// 		}
// 	}
// }

impl VisibleSymbol for backend::Terrain {
	fn visible_symbol(&self) -> char {
		match self {
			backend::Terrain::DeepWater => 'w',
			backend::Terrain::Ground => ' ',
			backend::Terrain::Wall => '#',
			backend::Terrain::ShallowWater => '~',
		}
	}
}

impl VisibleSymbol for backend::Species {
	fn visible_symbol(&self) -> char {
		match self {
			Species::Ay => 'a',
			Species::Bhederin => 'b',
			Species::Human => '@',
		}
	}
}

impl HiddenSymbol for backend::Terrain {
	fn hidden_symbol(&self) -> char {
		match self {
			backend::Terrain::DeepWater => self.visible_symbol(),
			backend::Terrain::Ground => ' ',
			backend::Terrain::Wall => self.visible_symbol(),
			backend::Terrain::ShallowWater => self.visible_symbol(),
		}
	}
}

// impl VisibleSymbol for backend::CharName {
// 	fn visible_symbol(&self) -> char {
// 		match self {
// 			backend::CharName::Ay => 'a',
// 			backend::CharName::Bhederin => 'b',
// 			backend::CharName::Human => 'h',
// 		}
// 	}
// }
