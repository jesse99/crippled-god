use super::*;
use std::ops::Add;
use std::ops::Sub;

/// Game time in (more or less) seconds.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Time {
	value: i64,
}

/// This is used to perform actions at a particular time for everything but the player.
pub trait Scheduled {
	/// Returns the time at which execute should be called.
	fn ready_time(&self) -> Time;

	/// Does some action and updates ready_time accordingly. Returns the new location or None if
	/// self should not be re-added to the level.
	fn execute(&mut self, level: &mut Level, loc: Location) -> Option<Location>;
}

impl Time {
	pub fn zero() -> Time {
		Time { value: 0 }
	}
}

impl Add<i64> for Time {
	type Output = Time;

	fn add(self, other: i64) -> Time {
		Time {
			value: self.value + other,
		}
	}
}

impl Sub<i64> for Time {
	type Output = Time;

	fn sub(self, other: i64) -> Time {
		Time {
			value: self.value - other,
		}
	}
}
