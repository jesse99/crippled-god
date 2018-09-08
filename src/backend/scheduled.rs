use std::ops::Add;
use std::ops::Sub;

/// Game time in (more or less) seconds.
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Time {
	value: i64,
}

/// This is used to perform actions at a particular time for everything but the player.
pub trait Scheduled {
	/// If self is ready to execute then this will do some action and return the next time at which
	///  self will be ready to do something. Otherwise this will return None.
	fn execute(&mut self, current_time: Time) -> Option<Time>;
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
