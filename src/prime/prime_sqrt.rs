pub mod prime_sqrt_mod {
	pub struct Factorize {
		n: u64, d: u64
	}
	
	impl Iterator for Factorize {
		type Item = (u64, usize);
	
		fn next(&mut self) -> Option<Self::Item> {
			while self.d.pow(2) <= self.n {
				let mut p = 0;
				while self.n % self.d == 0 { p += 1; self.n /= self.d; }
				self.d += 1;
				if p >= 1 { return Some((self.d-1, p)); }
			}
			if self.n > 1 {
				let item = Some((self.n, 1)); self.n = 1;
				return item;
			}
			None
		}
	}

	pub fn factorize(n: u64) -> Factorize { Factorize{n, d: 2} }

	pub fn is_prime(n: u64) -> bool {
		n >= 2 && factorize(n).next().unwrap().0 == n
	}
}
pub use prime_sqrt_mod::{factorize, is_prime};
