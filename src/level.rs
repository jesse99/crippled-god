use super::core;

/// Record of the terrain and positions of NPCs and items within a particular dungeon level.
pub struct Level {
	name: String,
	terrain: core::Vec2d<core::Terrain>,
}

// TODO:
// may want to store NPCs and items in their own services
// if we do that we may want to rename this Map
impl Level {
	/// Levels start out empty and become populated as events occur.
	pub fn new() -> Level {
		Level {
			name: "uninitialized".to_string(),
			terrain: core::Vec2d::empty(),
		}
	}

	// pub fn size(&self) -> Size {
	// 	self.terrain.size()
	// }

	pub fn on_event(&mut self, event: &core::Event, _queued: &mut core::QueuedEvents) {
		match event {
			core::Event::ResetLevel(name, size, terrain) => {
				self.name = name.to_string();
				self.terrain = core::Vec2d::new(*size, *terrain);
			}
			core::Event::SetTerrain(loc, terrain) => {
				self.terrain.set(*loc, *terrain);
			}
			_ => (),
		}
	}
}
