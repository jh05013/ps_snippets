pub mod rand_mod {
	/// A random number generator.
	pub struct Rng([u32; 4]);

	#[allow(clippy::as_conversions)]
	impl Rng {
		/// Initializes a new generator.
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			let mut seed = 0;
			// SAFETY: yes
			unsafe { std::arch::x86_64::_rdrand32_step(&mut seed); }
			Self::seeded(seed)
		}

		pub fn seeded(seed: u32) -> Self {
			let mut prev = seed;
			Self(std::array::from_fn(|_| {
				prev = Self::split_mix(prev); prev
			}))
		}

		const fn split_mix(v: u32) -> u32 {
			let mut z =  v.wrapping_add(0x9e37_79b9);
			z = (z ^ (z >> 15)).wrapping_mul(0x85eb_ca6b);
			z = (z ^ (z >> 13)).wrapping_mul(0xc2b2_ae35);
			z ^ (z >> 16)
		}
		#[allow(clippy::many_single_char_names)]
		fn gen32(&mut self) -> u32 {
			let [x, y, z, w] = &mut self.0;
			let res = x.wrapping_add(*w);
			let t = x.wrapping_shl(9);
			*y ^= *x; *w ^= *y; *y ^= *z; *x ^= *w; *z ^= t;
			*w = w.rotate_left(11);
			res
		}
		fn gen64(&mut self) -> u64 {
			u64::from(self.gen32()) << 32 | u64::from(self.gen32())
		}

		/// Generates an integer in `[0, n: u32)`.
		pub fn next_u32(&mut self, n: u32) -> u32 {
			assert!(n != 0, "Bad RNG bound 0");
			((u64::from(self.gen32()) * u64::from(n)) >> 32) as u32
		}
		/// Generates an integer in `[0, n: u64)`.
		pub fn next_u64(&mut self, n: u64) -> u64 {
			assert!(n != 0, "Bad RNG bound 0");
			((u128::from(self.gen64()) * u128::from(n)) >> 64) as u64
		}

		/// Shuffles the slice.
		pub fn shuffle<T>(&mut self, vals: &mut [T]) {
			assert!(vals.len() < u32::MAX as usize,
				"This is PS, why do you need such a long list??"
			);
			for i in 1..vals.len() {
				let j = self.next_u32(i as u32) as usize;
				vals.swap(i, j);
			}
		}
	}
}
pub use rand_mod::Rng;
