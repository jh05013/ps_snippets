//! Basic functions about primality and factorization.

/// A prime sieve trait.
/// 
/// This can be used to generate a [`Factorizer`], which factorizes
/// an input integer by trying each prime factor candidate.
/// 
/// # Example
/// ```
/// # use ps_snippets::math::prime::basic_sieve::*;
/// # use ps_snippets::math::prime::prime::PrimeSieve;
/// let sieve = BasicSieve::new(100);
/// assert_eq!(sieve.prime_limit(), 100);
/// assert!(sieve.is_prime(17));
/// assert!(!sieve.is_prime(24));
/// assert_eq!(sieve.primes().count(), 25);
/// ```
pub trait PrimeSieve {
	/// Returns the largest integer that the sieve has processed.
	fn prime_limit(&self) -> u64;
	/// Returns `true` iff `n` is a prime. Assume `n <= self.prime_limit()`.
	fn is_prime(&self, n: u64) -> bool;
	/// Returns an iterator of prime numbers up to `self.prime_limit()`.
	fn primes(&self) -> Box<dyn Iterator<Item = u64> + '_>;
}

/// See [`Factorizer::factorize`].
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

/// A utility to factorize integers.
/// 
/// To construct it, a type that implements [`PrimeSieve`] is required.
/// 
/// # Example
/// ```
/// # use ps_snippets::math::prime::basic_sieve::BasicSieve;
/// # use ps_snippets::math::prime::prime::Factorizer;
/// let fact = Factorizer::new(BasicSieve::new(10));
/// assert_eq!(fact.prime_limit, 10);
/// assert_eq!(fact.factorize_limit, 100);
/// assert_eq!(fact.primes, vec![2, 3, 5, 7]);
/// ```
#[derive(Clone, Default, Debug)]
pub struct Factorizer {
	/// The largest integer that the sieve has processed.
	pub prime_limit: u64,
	/// The largest integer that can be factorized, equal to the
	/// square of `prime_limit`.
	pub factorize_limit: u64,
	/// The list of all primes up to `prime_limit`.
	pub primes: Vec<u64>,
}

impl Factorizer {
	/// Creates a new factorizer instance.
	pub fn new(sieve: impl PrimeSieve) -> Self {
		let lim = sieve.prime_limit();
		Self {
			prime_limit: lim,
			factorize_limit: lim.saturating_pow(2),
			primes: sieve.primes().collect(),
		}
	}

	/// Returns an iterator of factorization of `n`.
	/// Each item is of the form `(p, d)`, where `p` is a prime factor
	/// and `d` is an exponent. Each `p` is distinct and in the
	/// ascending order.
	/// 
	/// If `n <= 1`, the iterator is empty.
	/// 
	/// 🕒 $O(\pi(\sqrt{n}))$ in total.
	/// 
	/// # Examples
	/// ```
	/// # use ps_snippets::math::prime::basic_sieve::BasicSieve;
	/// # use ps_snippets::math::prime::prime::Factorizer;
	/// let fact = Factorizer::new(BasicSieve::new(10));
	/// let mut terms = fact.factorize(100);
	/// assert_eq!(terms.next(), Some((2, 2)));
	/// assert_eq!(terms.next(), Some((5, 2)));
	/// assert_eq!(terms.next(), None);
	/// ```
	pub fn factorize(&self, n: u64) -> Factorization {
		assert!(n <= self.factorize_limit, "{n} is too large to factorize");
		Factorization { primes: &self.primes, n, pidx: 0 }
	}

	/// Returns the smallest prime factor of `n`, if `n >= 2`; otherwise
	/// returns [`None`].
	/// 
	/// 🕒 $O(\pi(\sqrt{n}))$.
	/// 
	/// # Examples
	/// ```
	/// # use ps_snippets::math::prime::basic_sieve::BasicSieve;
	/// # use ps_snippets::math::prime::prime::Factorizer;
	/// let fact = Factorizer::new(BasicSieve::new(10));
	/// assert_eq!(fact.first_factor(30), Some(2));
	/// assert_eq!(fact.first_factor(17), Some(17));
	/// assert_eq!(fact.first_factor(1), None);
	/// ```
	pub fn first_factor(&self, n: u64) -> Option<u64> {
		self.factorize(n).next().map(|(p, _)| p)
	}

	/// Returns `true` iff `n` is a prime number.
	/// 
	/// 🕒 $O(\pi(\sqrt{n}))$.
	/// 
	/// # Examples
	/// ```
	/// # use ps_snippets::math::prime::basic_sieve::BasicSieve;
	/// # use ps_snippets::math::prime::prime::Factorizer;
	/// let fact = Factorizer::new(BasicSieve::new(10));
	/// assert!(fact.is_prime(17));
	/// assert!(!fact.is_prime(20));
	/// assert!(!fact.is_prime(1));
	/// ```
	pub fn is_prime(&self, n: u64) -> bool {
		matches!(self.factorize(n).next(), Some((p, _)) if p == n)
	}
}
