use super::*;

/// Record of the terrain and positions of NPCs and items within a particular dungeon level.
pub struct Level {
	name: String,
	terrain: Vec2d<Terrain>,
}

// TODO:
// may want to store NPCs and items in their own services
// if we do that we may want to rename this Map
impl Level {
	/// Levels start out empty and become populated as events occur.
	pub fn new() -> Level {
		Level {
			name: "uninitialized".to_string(),
			terrain: Vec2d::empty(),
		}
	}

	pub fn size(&self) -> Size {
		self.terrain.size()
	}

	pub fn on_event(&mut self, events: &mut Events) {
		match events.last() {
			Event::InitLevel(name, size, terrain) => {
				self.name = name.to_string();
				self.terrain = Vec2d::new(*size, *terrain);
				events.append(Event::NewLevel);
			}
			Event::SetTerrain(loc, terrain) => {
				self.terrain.set(*loc, *terrain);
			}
			_ => (),
		}
	}
}
