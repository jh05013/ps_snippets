//! A segmented Eratosthenes sieve which is around 5 times faster
//! than [`BasicSieve`].
//! 
//! The trick is to generate prime numbers in batches of intervals,
//! making them cache-friendly and thus faster to sieve.
//! 
//! # Example
//! ```
//! # use ps_snippets::prime::segmented_sieve::*;
//! let sieve = SegmentedSieve::new(100);
//! let mut it = sieve.range_primes(10, 15);
//! assert_eq!(it.next(), Some(11));
//! assert_eq!(it.next(), Some(13));
//! assert_eq!(it.next(), None);
//! assert_eq!(sieve.primes().count(), 25);
//! ```
//! 
//! # Practice Problems
//! - [SPOJ TDPRIME Printing some primes](https://www.spoj.com/problems/TDPRIMES/) 1e8, 0.4s
//! - [LC Enumerate Primes](https://judge.yosupo.jp/problem/enumerate_primes) 5e8, 1.4s

/// A trait for generating prime numbers in a range.
pub trait RangePrime {
	/// The largest integer that the sieve has preprocessed.
	/// For [`SegmentedSieve`], this is roughly the square root of `gen_limit`.
	fn sieve_size(&self) -> usize;
	/// The largest integer that can be sieved.
	fn gen_limit(&self) -> u64;
	/// A function to iterate through all primes from `l` to `r`.
	fn range_primes(&self, l: u64, r: u64) -> Box<dyn Iterator<Item = u64> + '_>;

	/// A function to iterate through all primes from 2 to `gen_limit`.
	fn primes(&self) -> Box<dyn Iterator<Item = u64> + '_> {
		let n = self.sieve_size() as u64;
		let gl = self.gen_limit();
		Box::new(
			(0..n).flat_map(move |i| {
				let r = std::cmp::min(gl, n*i+n-1);
				self.range_primes(n*i, r)
			})
		)
	}
}

/// A segmented sieve.
#[derive(Clone, Debug, Default)]
pub struct SegmentedSieve {
	sieve_size: usize,
	n: u64,
	primes: Vec<usize>,
}

impl SegmentedSieve {
	/// Creates a new segmented sieve that can sieve up to `n`.
	pub fn new(n: u64) -> Self {
		let ssz = (((n as f64).sqrt() as u64) + 2) as usize;
		let mut sieve: Vec<bool> = vec![true; ssz+1];
		sieve[0] = false;
		if n != 0 { sieve[1] = false; }
		for p in 2..=n {
			let p = p as usize;
			if p*p > ssz { break; }
			if !sieve[p] { continue; }
			for q in (p*p..=ssz).step_by(p) { sieve[q] = false; }
		}
		Self {
			sieve_size: ssz,
			n,
			primes: (3..=ssz).filter(|&i| sieve[i]).collect::<Vec<_>>()
		}
	}
}

impl RangePrime for SegmentedSieve {
	fn sieve_size(&self) -> usize { self.sieve_size }

	fn gen_limit(&self) -> u64 { self.n }

	fn range_primes(&self, l: u64, r: u64) -> Box<dyn Iterator<Item = u64> + '_> {
		// offset
		let has_2 = if l <= 2 { Some(2) } else { None };
		let l = l/2*2;
		let r = (r+1)/2*2 - 1;
		if l > r { return Box::new(std::iter::empty()); }

		// init
		let sz = (r-l+1) as usize /2;
		let mut sieve: Vec<bool> = vec![true; sz];
		if l == 0 { sieve[1/2] = false; }

		for &p in &self.primes {
			let p = p as u64;
			let start = std::cmp::max(2, (l+p-1)/p)*p;
			let mut i = (start - l) as usize;
			if i%2 == 0 { i += p as usize; }
			i /= 2;
			while i < sz { sieve[i] = false; i += p as usize; }
		}
		Box::new(
			has_2.into_iter()
				.chain((0..sz)
					.filter(move |&i| sieve[i])
					.map(move |x| l+1+2*x as u64)
				)
		)
	}
}
