use std::fmt;
use std::i32; // for MAX
use std::ops::{Add, AddAssign};

/// Time at which a character (or item) will do something.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Time(pub i32); // this is large enough for 2400 days of play time

/// Amount of [`Time`] it takes to perform some action. Characters will not be able to do anything
/// until this time elapses.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Duration(pub i32);

pub const INFINITE_TIME: Time = Time(i32::MAX);

// /// Used to re-schedule an NPC when they decide to do nothing.
// pub const NO_OP_DURATION: Duration = Duration(10);

// pub const INFINITE_DURATION: Duration = Duration(i32::MAX);

/// Smallest unit of time: a tenth of a second.
// pub const TICK: Duration = Duration(1);

impl Time {
	// pub fn zero() -> Time {
	// 	Time(0)
	// }

	pub fn from_secs(secs: f32) -> Time {
		Time((secs * 10.0) as i32)
	}
}

impl Add<Duration> for Time {
	type Output = Time;

	fn add(self, rhs: Duration) -> Time {
		Time(self.0 + rhs.0)
	}
}

impl AddAssign<Duration> for Time {
	fn add_assign(&mut self, other: Duration) {
		*self = Time(self.0 + other.0);
	}
}

impl Duration {
	pub fn from_secs(secs: f32) -> Duration {
		Duration((secs * 10.0) as i32)
	}

	// 	pub fn percent(self, p: f64) -> Duration {
	// 		let x = (self.0 as f64) / 10.0 * p;
	// 		let x = (x * 10.0) as i32;
	// 		Duration(x)
	// 	}
}

impl fmt::Display for Time {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:.1}s", (self.0 as f32) / 10.0)
	}
}

impl fmt::Display for Duration {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:.1}s", (self.0 as f32) / 10.0)
	}
}
