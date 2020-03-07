use super::*;

/// This is used to record the facts that happened during a game.
/// Services will update their state as events come in. Events are
/// persisted so that games may be persisted and replayed for
/// debugging purposes (or to recover from a crash).
#[derive(Debug)]
pub enum Event {
	// Attacked(ID, ID, DamageType, DamageAmount, Duration),	// need details so UI can render stuff like bolts or big strikes
	/// Reset the current level with a name, size, and default terrain.
	InitLevel(String, Size, Terrain),

	/// Fires after InitLevel to allow services to finish initializing the level.
	NewLevel,
	// NewNPC(Point, ID, HPs),
	SetTerrain(Point, Terrain),
}
