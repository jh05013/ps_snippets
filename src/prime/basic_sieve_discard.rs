pub mod basic_sieve_discard_mod {
	/// Returns all primes up to `n` in order.
	pub fn primes(n: usize) -> Vec<usize> {
		let mut sieve = vec![true; n+1];
		for p in 2..=n {
			if p*p > n { break; }
			if !sieve[p] { continue; }
			for q in (p*p..=n).step_by(p) {
				sieve[q] = false;
			}
		}
		(2..=n).filter(|&x| sieve[x]).collect()
	}
} pub use basic_sieve_discard_mod::primes;
