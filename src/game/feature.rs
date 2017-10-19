use std::fmt;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Feature {
    DownStair,
    UpStair,
}

impl Feature {
    pub fn render(self, tile: &mut super::Tile) {
        match self {
            Feature::DownStair => {
                tile.symbol = '>';
                tile.fg = super::Color::Coral;
            }
            Feature::UpStair => {
                tile.symbol = '<';
                tile.fg = super::Color::SeaGreen;
            }
        }
    }

    pub fn passable(self) -> bool {
        match self {
            Feature::DownStair | Feature::UpStair => true,
        }
    }
}

impl fmt::Debug for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Feature::DownStair => write!(f, ">"),
            Feature::UpStair => write!(f, "<"),
        }
    }
}
