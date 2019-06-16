use std::i64;
use std::ops::Add;

/// Time at which a character (or item) will do something.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Time(pub i64); // stored as 100ths of a second

/// Amount of time it takes to perform some action. Characters will not be able to do anything
/// until this time elapses.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Duration(pub i64); // stored as 100ths of a second

/// This is a typical movement speed. Characters that are fast or slow will use a percentage of this.
pub const BASE_MOVEMENT_DURATION: Duration = Duration(500);

/// Used to re-schedule an NPC when they decide to do nothing.
pub const NO_OP_DURATION: Duration = Duration(100);

pub const INFINITE_DURATION: Duration = Duration(i64::MAX);

impl Time {
	pub fn zero() -> Time {
		Time(0)
	}

	pub fn from_seconds(secs: i64) -> Time {
		Time(secs*100)
	}
}

impl Add<Duration> for Time {
	type Output = Time;

	fn add(self, rhs: Duration) -> Time {
		Time(self.0 + rhs.0)
	}
}

impl Duration {
	pub fn from_seconds(secs: i64) -> Duration {
		Duration(secs*100)
	}

	pub fn percent(self, p: f64) -> Duration {
		let x = (self.0 as f64) / 100.0 * p;
		let x = (x * 100.0) as i64;
		Duration(x)
	}
}