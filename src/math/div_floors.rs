pub mod div_floors_mod {
	/// See [`div_floors`].
	pub struct DivFloors { n: u64, x: u64, last: u64 }

	/// Returns an iterator of the values of `n/x` for a given `n`.
	/// 
	/// Each item is of the form `(k, l, r)`, which denotes that
	/// `n/x == k` for x in $[l, r]$. The items are given in the
	/// decreasing order of `k`.
	pub fn div_floors(n: u64) -> DivFloors {
		if n == 0 { DivFloors { n, x: 0, last: 1 } }
		else { DivFloors { n, x: 1, last: 0 } }
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
}
pub use div_floors_mod::div_floors;
