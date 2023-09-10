//! Prime-related functions based on sqrt-time primality testing.
//! 
//! It's obvously slow. The purpose of this module is for assessing
//! a problem's difficulty by checking how slow the algorithms can
//! be while still getting AC.
//! 
//! # Practice Problems
//! - [PE 3 Largest Prime Factor](https://projecteuler.net/problem=3) `factorize`
//! - [BOJ 11653 소인수분해](https://www.acmicpc.net/problem/11653) `factorize`
//! - [PE 10 Summation of Primes](https://projecteuler.net/problem=10) `is_prime`
//! - [BOJ 4134 다음 소수](https://www.acmicpc.net/problem/4134) `next_prime`
//! - [PE 7 10001st Prime](https://projecteuler.net/problem=7) `nth_prime`

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

/// Returns a factorization iterator.
/// 
/// Each item is of the form `(p, d)`, meaning that the
/// factorization of `n` contains $p^d$.
/// 
/// 🕒 $(O(\sqrt n))$ until the end.
/// 
/// # Example
/// ```
/// use ps_snippets::math::prime_sqrt::*;
/// 
/// let mut fac = factorize(20);
/// assert_eq!(fac.next(), Some((2, 2)));
/// assert_eq!(fac.next(), Some((5, 1)));
/// assert_eq!(fac.next(), None);
/// ```
pub fn factorize(n: u64) -> Factorize {
	Factorize { n, d: 2 }
}

/// Returns `true` if `n` is a prime, otherwise `false`.
/// 
/// 🕒 $(O(\sqrt n))$.
pub fn is_prime(n: u64) -> bool {
	matches!(factorize(n).next(), Some((z, _)) if z == n)
}

/// Returns the first prime larger than `n`.
/// 
/// 🕒 see <https://en.wikipedia.org/wiki/Prime_gap#Numerical_results>
pub fn next_prime(mut n: u64) -> u64 {
	while !is_prime(n+1) { n += 1; }
	n+1
}

/// Returns the `n`-th prime. `n` is 1-based, so `nth_prime(1) == 2`.
/// 
/// 🕒 $(O(n \sqrt n \log n))$ (see <https://en.wikipedia.org/wiki/Prime_number_theorem>)
/// 
/// ⚠️ Panics if `n == 0`.
pub fn nth_prime(n: usize) -> u64 {
	assert!(n != 0, "nth_prime(0) is not allowed");
	let mut p = 2;
	for _ in 1..n { p = next_prime(p); }
	p
}
