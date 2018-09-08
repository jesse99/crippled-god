use super::*;
use backend;
use termion;

/// Visual representation of terrain, items, and characters on a position within the map.
pub struct View {
	pub symbol: char,
	pub fg: termion::color::AnsiValue,
	pub bg: termion::color::AnsiValue,
	// TODO: might want to add support for styles, see https://docs.rs/termion/1.5.1/termion/style/index.html
}

impl View {
	pub fn new(tile: &backend::Tile) -> View {
		if tile.visible {
			match tile.character {
				backend::CharacterType::Player(_) => {
					let bg = colors::to_termion(tile.terrain.back_color());
					let fg = colors::to_termion(colors::Color::White);
					let symbol = '@'; // TODO: use player.race
					View { symbol, fg, bg }
				}
				backend::CharacterType::NPC(species) => {
					let bg = colors::to_termion(tile.terrain.back_color());
					let fg = colors::to_termion(species.fore_color());
					let symbol = species.visible_symbol();
					View { symbol, fg, bg }
				}
				backend::CharacterType::None => {
					let bg = colors::to_termion(tile.terrain.back_color());
					let fg = colors::to_termion(tile.terrain.fore_color());
					let symbol = tile.terrain.visible_symbol();
					View { symbol, fg, bg }
				}
			}
		} else {
			match tile.character {
				backend::CharacterType::Player(_) => {
					let bg = colors::to_termion(colors::Color::LightGrey);
					let fg = colors::to_termion(colors::Color::White);
					let symbol = '@'; // TODO: use player.race
					View { symbol, fg, bg }
				}
				backend::CharacterType::NPC(species) => {
					let bg = colors::to_termion(colors::Color::LightGrey);
					let fg = colors::to_termion(colors::Color::DarkGray);
					let symbol = species.visible_symbol();
					View { symbol, fg, bg }
				}
				backend::CharacterType::None => {
					let bg = colors::to_termion(colors::Color::LightGrey);
					let fg = colors::to_termion(colors::Color::DarkGray);
					let symbol = tile.terrain.hidden_symbol();
					View { symbol, fg, bg }
				}
			}
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
			backend::Terrain::Blank => colors::Color::Black,
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
			backend::Terrain::Blank => colors::Color::Black,
			backend::Terrain::DeepWater => colors::Color::Blue,
			backend::Terrain::Ground => colors::Color::LightSlateGray,
			backend::Terrain::Wall => colors::Color::Chocolate,
			backend::Terrain::ShallowWater => colors::Color::Blue,
		}
	}
}

impl ToForeColor for backend::Species {
	fn fore_color(&self) -> colors::Color {
		match self {
			backend::Species::Ay => colors::Color::BurlyWood,
			backend::Species::Bison => colors::Color::Chocolate,
		}
	}
}

impl VisibleSymbol for backend::Terrain {
	fn visible_symbol(&self) -> char {
		match self {
			backend::Terrain::Blank => '?',
			backend::Terrain::DeepWater => 'w',
			backend::Terrain::Ground => ' ',
			backend::Terrain::Wall => '#',
			backend::Terrain::ShallowWater => '~',
		}
	}
}

impl HiddenSymbol for backend::Terrain {
	fn hidden_symbol(&self) -> char {
		match self {
			backend::Terrain::Blank => ' ',
			backend::Terrain::DeepWater => self.visible_symbol(),
			backend::Terrain::Ground => ' ',
			backend::Terrain::Wall => self.visible_symbol(),
			backend::Terrain::ShallowWater => self.visible_symbol(),
		}
	}
}

impl VisibleSymbol for backend::Species {
	fn visible_symbol(&self) -> char {
		match self {
			backend::Species::Ay => 'a',
			backend::Species::Bison => 'b',
		}
	}
}
