/// A Barrett reduction interface.
pub mod barrett {
	#[derive(Debug, Clone)]
	pub struct Barrett { n: u64, m: u128 }
	
	impl Barrett {
		/// Creates a new Barrett reduction interface modulo `n`.
		pub fn new(n: u64) -> Self {
			Barrett { n, m: (1u128 << 64) / n as u128 }
		}
	
		/// Returns `x` mod `n`.
		pub fn reduce(&self, x: u64) -> u64 {
			let q = ((self.m * (x as u128)) >> 64) as u64;
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
pub use barrett::Barrett;
