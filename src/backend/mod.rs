mod geography;
mod level;
mod location;
mod player;
mod pov;
mod size;
mod terrain;
mod vec2;

pub use self::geography::Geography;
pub use self::level::Cell;
pub use self::level::Character;
pub use self::level::Level;
pub use self::location::Location;
pub use self::player::Player;
pub use self::pov::*;
pub use self::size::Size;
pub use self::terrain::Terrain;

pub fn silly() {
	field_of_view(Location::new(0, 0), Size::mew(0, 0), 10, 1, 2);
}
