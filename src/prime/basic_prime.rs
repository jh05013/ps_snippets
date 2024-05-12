pub mod basic_primes_mod {
	/// Returns a vector of length `n+1` whose `[i]` is true iff `i` is prime.
	/// `n` IS included!
	pub fn prime_sieve(n: usize) -> Vec<bool> {
		let mut sieve = vec![true; n+1];
		sieve[0] = false;
		if n >= 1 { sieve[1] = false; }
		for p in 2..=n {
			if p*p > n { break; }
			if !sieve[p] { continue; }
			for q in (p*p..=n).step_by(p) {
				sieve[q] = false;
			}
		}
		sieve
	}

	/// Returns all primes up to `n` in order. `n` IS included!
	pub fn primes(n: usize) -> Vec<usize> {
		prime_sieve(n).into_iter().enumerate()
			.filter(|(_, b)| *b).map(|(p, _)| p).collect()
	}
} pub use basic_primes_mod::{prime_sieve, primes};

pub mod basic_primality_mod {
	/// Returns true iff `n` is prime.
	pub fn is_prime(n: u64) -> bool {
		for p in 2..=n {
			if p*p > n { return true; }
			if n%p == 0 { return false; }
		}
		n >= 2
	}

	/// Returns the `n`-th prime, starting from 0th = 2.
	pub fn nth_prime(n: usize) -> u64 {
		(2..).filter(|x| is_prime(*x)).nth(n).unwrap()
	}
} pub use basic_primality_mod::{is_prime, nth_prime};

pub mod basic_factor_mod {
	pub struct Factorize { n: u64, d: u64 }
	
	impl Iterator for Factorize { type Item = (u64, usize);
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

	/// Returns the prime factorization of `n`, expressed as
	/// an iterator of (prime factor, multiplicity).
	/// Prime factors are given in increasing order.
	pub fn factorize(n: u64) -> Factorize { Factorize{n, d: 2} }
}
pub use basic_factor_mod::{Factorize, factorize};
