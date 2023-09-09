//! Iterator for a floor division of `n`.
//! 
//! It can be proven that for a non-negative integer $ N $, the expression $ \lfloor \frac{N}{x} \rfloor $ for integer $ x $ can have $ O(\sqrt{N}) $ different values.
//! # Practice Problems
//! - [LC Enumerate Quotients](https://judge.yosupo.jp/problem/enumerate_quotients) 1e12, 108 ms
//! - [BOJ 26056 수열의 합 2](https://www.acmicpc.net/problem/26056) 1e14, twice in 436 ms

/// See [`div_floors`].
pub struct DivFloors { n: u64, x: u64, last: u64 }

/// An iterator of the values of `n/x` for a given `n`.
/// 
/// Each item is of the form `(k, l, r)`, which denotes that
/// `n/x == k` for x in $[l, r]$. The items are given in the
/// decreasing order of `k`.
/// 
/// The implementation looks convoluted, but the purpose is to make
/// it as fast as possible by doing as few divisions as possible
/// (once per item).
/// 
/// 🕒 $(O(\sqrt n))$ until the end.
/// 
/// # Example
/// ```
/// use ps_snippets::math::div_floors::*;
/// 
/// /*
/// x    1 2 3 4 5 6 7 8 9 10 11 12 13
/// ------------------------------------
/// N/x 10 5 3 2 2 1 1 1 1  1  0  0  0
/// */
/// let mut df = div_floors(10);
/// assert_eq!(df.next(), Some((10, 1, 1)));
/// assert_eq!(df.next(), Some((5, 2, 2)));
/// assert_eq!(df.next(), Some((3, 3, 3)));
/// assert_eq!(df.next(), Some((2, 4, 5)));
/// assert_eq!(df.next(), Some((1, 6, 10)));
/// assert_eq!(df.next(), None);
/// ```
/// 
pub fn div_floors(n: u64) -> DivFloors {
	DivFloors { n, x: 1, last: 0 }
}

impl Iterator for DivFloors {
	type Item = (u64, u64, u64);
	fn next(&mut self) -> Option<Self::Item> {
		let (n, x) = (self.n, self.x);
		if self.last == 0 {
			let item = Some((n/x, x, x));
			self.x += 1;
			if self.x.pow(2) > n { self.last = self.x - 1; self.x = n/self.x; }
			return item;
		}
		if x == 0 { return None; }
		let new_last = n/x;
		let item = Some((x, self.last + 1, new_last));
		self.last = new_last;
		self.x -= 1;
		item
	}
}
