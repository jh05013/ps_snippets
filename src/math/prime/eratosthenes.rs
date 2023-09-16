use super::prime::*;

#[derive(Clone, Default, Debug)]
pub struct EratosthenesSieve {
	pub sieve: Vec<bool>,
}

impl EratosthenesSieve {
	pub fn new(n: usize) -> Self {
		let mut sieve = vec![true; n+1];
		sieve[0] = false;
		sieve[1] = false;
		for p in 2..=n {
			if p*p > n { break; }
			if !sieve[p] { continue; }
			for q in (p*p..=n).step_by(p) { sieve[q] = false; }
		}
		Self { sieve }
	}

	pub fn n(&self) -> usize { self.sieve.len() - 1 }
}

impl PrimeSieve for EratosthenesSieve {
	fn prime_limit(&self) -> u64 { self.n() as u64 }

	fn is_prime(&self, n: u64) -> bool { self.sieve[n as usize] }

	fn primes(&self) -> Box<dyn Iterator<Item = u64> + '_> {
		Box::new(
			(0..=self.prime_limit()).filter(|&n| self.sieve[n as usize])
		)
	}
}
