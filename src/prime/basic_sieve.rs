//! A minimal implementation of Eratosthenes sieve in
//! $O(n \log \log n)$ time.
//! 
//! However, this is not the fastest implementation possible.
//! The purpose of this module is for assessing a problem's
//! difficulty by checking how slow the algorithms can be while
//! still getting AC.
//! 
//! If you need a faster implementation, use (TODO).

use super::prime::*;

/// A simple Eratosthenes sieve.
/// 
/// This implements [`PrimeSieve`].
/// 
/// # Example
/// ```rust
/// # use ps_snippets::prime::basic_sieve::*;
/// # use ps_snippets::prime::prime::PrimeSieve;
/// let sieve = BasicSieve::new(100);
/// assert_eq!(sieve.lp[17], 17);
/// assert_eq!(sieve.lp[24], 2);
/// assert_eq!(sieve.prime_limit(), 100);
/// assert!(sieve.is_prime(17));
/// assert!(!sieve.is_prime(24));
/// assert_eq!(sieve.primes().count(), 25);
/// ```
/// 
/// # Practice Problems
/// - [PE 10 Summation of Primes](https://projecteuler.net/problem=10) 1e6
/// - [BOJ 1929 소수 구하기](https://www.acmicpc.net/problem/1929) 1e6
/// - [SPOJ APS Amazing Prime Sequence](https://www.spoj.com/problems/APS/) 1e7, `lp`
#[derive(Clone, Default, Debug)]
pub struct BasicSieve {
	/// `lp[i]` is the smallest prime factor of `i` for `i >= 2`.
	/// Also, `lp[0] == 0 && lp[1] == 0`.
	pub lp: Vec<u32>,
}

impl BasicSieve {
	/// Creates a new prime sieve of size `n`.
	/// 
	/// 🕒 $O(n \log \log n)$
	pub fn new(n: u32) -> Self {
		let mut lp = (0..=n).collect::<Vec<_>>();
		if n != 0 { lp[1] = 0; }
		for p in 2..=n {
			if p*p > n { break; }
			if lp[p as usize] != p { continue; }
			for q in (p*p..=n).step_by(p as usize) {
				if lp[q as usize] == q { lp[q as usize] = p; }
			}
		}
		Self { lp }
	}
}

impl PrimeSieve for BasicSieve {
	/// 🕒 $O(1)$.
	fn prime_limit(&self) -> u64 { (self.lp.len() - 1) as u64 }
	/// 🕒 $O(1)$.
	fn is_prime(&self, n: u64) -> bool { n != 0 && self.lp[n as usize] == n as u32 }
	/// 🕒 $O(n)$ in total.
	fn primes(&self) -> Box<dyn Iterator<Item = u64> + '_> {
		Box::new(
			(2..=self.prime_limit()).filter(move |&n| self.is_prime(n))
		)
	}
}
