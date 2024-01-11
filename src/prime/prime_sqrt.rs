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
} use prime_sqrt_mod::{factorize};
