/// A Barrett reduction interface.
pub mod barrett_mod {
	#[derive(Debug, Clone)]
	pub struct Barrett { n: u64, m: u128 }
	
	impl Barrett {
		/// Creates a new Barrett reduction interface modulo `n`.
		pub fn new(n: u64) -> Self {
			Self { n, m: (1u128 << 64) / u128::from(n) }
		}
	
		/// Returns `x` mod `n`.
		#[allow(clippy::as_conversions)]
		pub fn reduce(&self, x: u64) -> u64 {
			let q = ((self.m * u128::from(x)) >> 64) as u64;
			let r = x - q * self.n;
			if r >= self.n { r - self.n } else { r }
		}
	
		/// Returns $x^k$ mod `n`.
		pub fn modpow(&self, mut x: u64, mut k: u64) -> u64 {
			let mut ans = 1u64;
			while k != 0 {
				if (k&1) != 0 { ans = self.reduce(ans * x); }
				k>>= 1; x = self.reduce(x * x);
			}
			ans
		}
	}
}
pub use barrett_mod::Barrett;
