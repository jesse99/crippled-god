/// Visual representation of terrain, items, and characters on a position within the map.
pub struct Tile {
    pub symbol: char,
    pub fg: super::Color,
    pub bg: super::Color,
	// TODO: might want to add support for styles, see https://docs.rs/termion/1.5.1/termion/style/index.html
}
