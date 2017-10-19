use std::fmt;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Terrain {
    Ground,
    Water,
    Wall,
}

impl Terrain {
    pub fn render(self) -> super::Tile {
        match self {
            Terrain::Ground => {
                super::Tile {
                    symbol: ' ',
                    fg: super::Color::LightGrey,
                    bg: super::Color::LightGrey,
                }
            }
            Terrain::Water => {
                super::Tile {
                    symbol: ' ',
                    fg: super::Color::DodgerBlue,
                    bg: super::Color::DodgerBlue,
                }
            }
            Terrain::Wall => {
                super::Tile {
                    symbol: ' ',
                    fg: super::Color::SaddleBrown,
                    bg: super::Color::SaddleBrown,
                }
            }
        }
    }

    pub fn passable(self) -> bool {
        // TODO: should take a reference to the player or NPC (or maybe capabilities)
        match self {
            Terrain::Ground | Terrain::Water => true,
            Terrain::Wall => false,
        }
    }
}

impl fmt::Debug for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terrain::Ground => write!(f, "."),
            Terrain::Water => write!(f, "w"),
            Terrain::Wall => write!(f, "#"),
        }
    }
}
