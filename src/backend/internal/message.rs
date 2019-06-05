
use super::super::Game;
use super::*;
/// Used with Message.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Topic {
	/// An operation could not be completed.
	Error,

	/// Something that doesn't affect the game.
	NonGamePlay,

	/// NPC was damaged (but not by the player).
	NpcIsDamaged, // TODO: might want to have a separate Topic for player allies

	/// NPC was attacked but not damaged (but not by the player).
	NpcIsNotDamaged,

	/// The player has caused damage.
	PlayerDidDamage,

	/// The player attacked but did no damage.
	PlayerDidNoDamage,

	/// The player has taken damage.
	PlayerIsDamaged,

	/// The player was attacked but took no damage.
	PlayerIsNotDamaged,

	/// The player will operate less well.
	PlayerIsImpaired, // TODO: probably also want a PlayerEnchanced

	/// The player is at risk of taking damage.
	PlayerIsThreatened,

	/// An operation was not completely successful.
	Warning,
}

pub struct Message {
	pub topic: Topic,
	pub text: String,
}

pub trait MessageFor {
	/// Message emitted when an entity interacts with something, e.g. moving into some terrain.
	fn message_for(&self, game: &Game, entity: Entity) -> Option<Message>;
}

