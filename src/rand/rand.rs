pub mod rand_mod {
	/// A random [`u32`] generator.
	pub struct Rng32([u32; 4]);

	impl Rng32 {
		const fn split_mix(v: u32) -> u32 {
			let mut z =  v.wrapping_add(0x9e37_79b9);
			z = (z ^ (z >> 15)).wrapping_mul(0x85eb_ca6b);
			z = (z ^ (z >> 13)).wrapping_mul(0xc2b2_ae35);
			z ^ (z >> 16)
		}

		/// Initializes a new generator.
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			let mut seed = 0;
			unsafe { std::arch::x86_64::_rdrand32_step(&mut seed) };
			let mut prev = seed;
			Self(std::array::from_fn(|_| {
				prev = Self::split_mix(prev);
				prev
			}))
		}

		/// Generates an integer in `[0, n)`.
		#[allow(clippy::many_single_char_names)]
		pub fn next(&mut self, n: u32) -> u32 {
			assert!(n != 0, "Bad RNG bound 0");
			let [x, y, z, w] = &mut self.0;
			let res = x.wrapping_add(*w);
			let t = x.wrapping_shl(9);
			*y ^= *x;
			*w ^= *y;
			*y ^= *z;
			*x ^= *w;
			*z ^= t;
			*w = w.rotate_left(11);
			((u64::from(res) * u64::from(n)) >> 32) as u32
		}
	}
	
	/// A random [`u64`] generator.
	pub struct Rng64([u64; 4]);
	
	impl Rng64 {
		const fn split_mix(v: u64) -> u64 {
			let mut z =  v.wrapping_add(0x9e37_79b9_7f4a_7c15);
			z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
			z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
			z ^ (z >> 31)
		}

		/// Initializes a new generator.
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			let mut seed = 0;
			unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };
			let mut prev = seed;
			Self(std::array::from_fn(|_| {
				prev = Self::split_mix(prev);
				prev
			}))
		}

		/// Generates an integer in `[0, n)`.
		#[allow(clippy::many_single_char_names)]
		pub fn next(&mut self, n: u64) -> u64 {
			assert!(n != 0, "Bad RNG bound 0");
			let [x, y, z, w] = &mut self.0;
			let res = x.wrapping_add(*w);
			let t = x.wrapping_shl(17);
			*y ^= *x;
			*w ^= *y;
			*y ^= *z;
			*x ^= *w;
			*z ^= t;
			*w = w.rotate_left(45);
			((u128::from(res) * u128::from(n)) >> 64) as u64
		}
	}
}
pub use rand_mod::{Rng32, Rng64};
