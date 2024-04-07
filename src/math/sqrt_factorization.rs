pub mod sqrt_factorization {
	pub fn factorize(mut n: u64) -> Vec<(u64, usize)> {
		let mut ans = vec![];
		for d in 2.. {
			if d*d > n { break; }
			let mut k = 0;
			while n%d == 0 { n/= d; k+= 1; }
			if k > 0 { ans.push((d, k)); }
		}
		if n != 1 { ans.push((n, 1)); }
		ans
	}
} pub use sqrt_factorization::factorize;
