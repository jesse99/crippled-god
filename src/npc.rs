use super::core::*;
// use super::level::*;

pub trait NPC {
	fn ready_time(&self) -> Time;
	fn loc(&self) -> Point;

	fn on_event(&mut self, event: &Event, queued: &mut QueuedEvents);
}
