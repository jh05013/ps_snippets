pub mod div_floors {
	/// See [`div_floors`].
	pub struct DivFloors { n: u64, x: u64, last: u64 }

	/// Returns an iterator of the values of `n/x` for a given `n`.
	/// 
	/// Each item is of the form `(k, l, r)`, which denotes that
	/// `n/x == k` for x in $[l, r]$. The items are given in the
	/// decreasing order of `k`.
	pub const fn div_floors(n: u64) -> DivFloors {
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
pub use div_floors::div_floors;

pub mod div_ceils {
	pub const fn div_ceil(n: u64, x: u64) -> u64 {
		n/x + if n%x == 0 { 0 } else { 1 }
	}

	pub struct DivCeils { n: u64, x: u64, last: u64 }

	pub const fn div_ceils(n: u64) -> DivCeils {
		if n <= 1 { DivCeils { n, x: 1, last: 1 } }
		else { DivCeils { n, x: 1, last: 0 } }
	}
	
	impl Iterator for DivCeils {
		type Item = (u64, u64, u64);
		fn next(&mut self) -> Option<Self::Item> {
			let (n, x) = (self.n, self.x);
			if self.last == 0 {
				let item = Some((div_ceil(n, x), x, x));
				self.x += 1;
				if self.x.pow(2) > n {
					self.last = self.x - 1;
					self.x = div_ceil(n, self.x);
				}
				return item;
			}
			if x == 1 { return None; }
			let new_last = div_ceil(n, x-1) - 1;
			let item = Some((x, self.last + 1, new_last));
			(self.x, self.last) = (x-1, new_last);
			item
		}
	}
}
pub use div_ceils::{div_ceil, div_ceils};
