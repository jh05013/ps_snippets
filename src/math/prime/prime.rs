pub trait PrimeSieve {
	fn prime_limit(&self) -> u64;
	fn is_prime(&self, n: u64) -> bool;
	fn primes(&self) -> Box<dyn Iterator<Item = u64> + '_>;
}

pub struct Factorization<'a> {
	primes: &'a Vec<u64>,
	n: u64, pidx: usize,
}

impl<'a> Iterator for Factorization<'a> {
	type Item = (u64, usize);
	fn next(&mut self) -> Option<Self::Item> {
		if self.n <= 1 { return None; }
		while self.pidx < self.primes.len() {
			let mut d = 0;
			let p = self.primes[self.pidx];
			if p*p > self.n { break; }
			while self.n % p == 0 { d += 1; self.n /= p; }
			if d > 0 { return Some((p, d)); }
			self.pidx += 1;
		}
		if self.n > 1 { Some((std::mem::take(&mut self.n), 1)) }
		else { None }
	}
}

#[derive(Clone, Default, Debug)]
pub struct Factorizer {
	pub prime_limit: u64,
	pub factorize_limit: u64,
	pub primes: Vec<u64>,
}

impl Factorizer {
	pub fn new(sieve: impl PrimeSieve) -> Self {
		let lim = sieve.prime_limit();
		Self {
			prime_limit: lim,
			factorize_limit: lim.saturating_pow(2),
			primes: sieve.primes().collect(),
		}
	}

	pub fn factorize(&self, n: u64) -> Factorization {
		assert!(n <= self.factorize_limit, "{n} is too large to factorize");
		Factorization { primes: &self.primes, n, pidx: 0 }
	}

	pub fn first_factor(&self, n: u64) -> Option<u64> {
		self.factorize(n).next().map(|(p, _)| p)
	}

	pub fn is_prime(&self, n: u64) -> bool {
		self.factorize(n).next().is_none()
	}
}
