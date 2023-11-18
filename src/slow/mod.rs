//! Slow utilities for test-solving.
//! 
//! The purpose of these functions are to assess a problem's
//! difficulty by checking how slow the algorithms can be
//! while still getting AC. Do not use this for competitive
//! purpose.

/// Returns the list of divisors of `n`, not sorted. $O(\sqrt(n))$.
pub fn divisors(n: u32) -> Vec<u32> {
	let mut divs = vec![];
	for d in 1..=n {
		if d*d > n { return divs; }
		if d*d == n { divs.push(d); }
		else if n%d == 0 { divs.append(&mut vec![d, n/d]); }
	}
	divs
}
