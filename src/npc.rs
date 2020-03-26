use super::core::*;
use super::level::*;
use super::player::*;

pub trait NPC {
	fn ready_time(&self) -> Time;
	fn loc(&self) -> Point;

	fn on_event(
		&mut self,
		event: &Event,
		queued: &mut QueuedEvents,
		level: &Level,
		player: &Player,
	);
}
