// Based on https://docs.rs/crate/random/0.12.2 with the following changes:
// 1) Deserialize, Serialize has been added.
// 2) Doesn't panic if zero is used for the seed.
// 3) Seed is a u64 instead of a [u64; 2].
// 5) Doesn't use the Source trait.
//#[derive(Clone, Copy, Deserialize, Serialize)]
#[derive(Clone, Copy)]
pub struct RNG(u64, u64);

impl RNG {
	/// Create a new random number generator.
	#[inline(always)]
	pub fn new(seed: u64) -> RNG {
		if seed == 0 {
			RNG(43, 1)
		} else {
			RNG(42, seed)
		}
	}

	/// Return `u64` uniformly distributed over `{0, 1, …, u64::MAX}`.
	#[inline(always)]
	fn read_u64(&mut self) -> u64 {
		let (mut x, y) = (self.0, self.1);
		self.0 = y;
		x = x ^ (x << 23);
		x = x ^ (x >> 17);
		x = x ^ y ^ (y >> 26);
		self.1 = x;
		x.wrapping_add(y)
	}

	/// Return `f64` uniformly distributed over `[0, 1]`.
	#[inline(always)]
	fn read_f64(&mut self) -> f64 {
		self.read_u64() as f64 / ::std::u64::MAX as f64
	}

	/// Return a random value.
	#[inline(always)]
	pub fn read<V>(&mut self) -> V
	where
		Self: Sized,
		V: Value,
	{
		Value::read(self)
	}

	// If we want an iter() for a random sequence then we can add the iter function (and the
	// Sequence trait, see https://docs.rs/random/0.12.2/src/random/sequence.rs.html#6-9).

	// From https://docs.rs/rand/0.5.5/src/rand/lib.rs.html#413-723
	// pub fn shuffle<T>(&mut self, values: &mut [T]) {
	// 	let mut i = values.len();
	// 	while i >= 2 {
	// 		// invariant: elements with index >= i have been locked in place.
	// 		i -= 1;
	// 		// lock element i in place.
	// 		let j = self.read::<usize>() % (i + 1);
	// 		values.swap(i, j);
	// 	}
	// }
}

/// Trait used to instantiate RNGs for all the float and integral types.
pub trait Value {
	/// Read a random value.
	fn read(rng: &mut RNG) -> Self;
}

macro_rules! implement(
    ($reader:ident as $($kind:ty),*) => {
        $(impl Value for $kind {
            #[inline(always)]
            fn read(source: &mut RNG) -> Self {
                source.$reader() as $kind
            }
        })*
    }
);

implement!(read_f64 as f32, f64);
implement!(read_u64 as i8, i16, i32, i64, isize);
implement!(read_u64 as u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn read() {
		let mut source = RNG::new(69);

		macro_rules! read(
            ($($kind:ident => [$one:expr, $two:expr],)*) => ({$(
                assert_eq!(source.read::<$kind>(), $one);
                assert_eq!($kind::read(&mut source), $two);
            )*});
        );

		read! {
			i8 => [52, -34],
			i16 => [-17348, -1036],
			i32 => [948_125_133, -1_432_682_055],
			i64 => [-6_330_235_019_914_458_621, -4_877_218_639_256_617_945],
		}
	}
}
