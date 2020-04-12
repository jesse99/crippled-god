use super::*;
use std::fmt;

/// This is used to record the facts that happened during a game.
/// Services will update their state as events come in. Events are
/// persisted so that games may be persisted and replayed for
/// debugging purposes (or to recover from a crash).
#[derive(Debug)]
pub enum Event {
	AdvanceTime(Time),
	// Attacked(ID, ID, DamageType, DamageAmount, Duration),	// need details so UI can render stuff like bolts or big strikes
	/// First event that fires when player enters a brand new level.
	NewBranch, // TODO: probably want to include a branch name
	NewGame,
	/// Fires after level is initialized to allow services to finish initializing.
	NewLevel,
	// NewNPC(Point, ID, HPs),
	/// Update the current level with a name, size, and default terrain.
	/// SetTerrain events will follow this.
	ResetLevel(String, Size, Terrain),

	SetPlayer(Point),

	SetTerrain(Point, Terrain),
}

impl fmt::Display for Event {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Event::AdvanceTime(t) => write!(f, "AdvanceTime({})", t),
			Event::NewBranch => write!(f, "NewBranch"),
			Event::NewGame => write!(f, "NewGame"),
			Event::NewLevel => write!(f, "NewLevel"),
			Event::ResetLevel(n, s, t) => write!(f, "ResetLevel({}, {}, {})", n, s, t),
			Event::SetPlayer(l) => write!(f, "SetPlayer({})", l),
			Event::SetTerrain(l, t) => write!(f, "SetTerrain({}, {})", l, t),
		}
	}
}
