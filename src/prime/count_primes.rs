pub mod count_primes {
	/// Returns the number of primes <= n. Roughly O(n^(3/4)).
	/// 
	/// Reference: https://codeforces.com/blog/entry/91632
	pub fn count_primes(n: u64) -> u64 {
		let mut quo: Vec<u64> = vec![];
		let sqrt = {
			let mut k = 1;
			loop {
				if k*k > n { break k-1; }
				else if k*k == n { break k; }
				quo.push(n/k); k += 1;
			}
		};
		let quo = (1..=sqrt).chain(quo.into_iter().rev()).collect::<Vec<_>>();
		
		// get(x) is i s.t quo[i] == x
		let get = |x: u64| {
			(if x <= sqrt { x-1 } else { quo.len() as u64 - n/x }) as usize
		};

		let mut dp = quo.clone();
		let mut a = 0;
		for p in 2..=sqrt {
			if dp[p as usize-1] == dp[p as usize-2] { continue; }
			a += 1;
			for (i, &q) in quo.iter().enumerate().rev() {
				if q < p*p { break; }
				dp[i] -= dp[get(q/p)] - a;
			}
		}
		dp[get(n)]-1
	}
}
pub use count_primes::count_primes;
