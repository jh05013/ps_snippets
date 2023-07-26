pub struct DivFloors { n: u64, x: u64, last: u64 }
pub fn div_floors(n: u64) -> DivFloors { DivFloors { n, x: 1, last: 0 } }
// (v,l,r) where n/x = v for x in [l, r]
impl Iterator for DivFloors {
	type Item = (u64, u64, u64);
	fn next(&mut self) -> Option<Self::Item> {
		let (n, x) = (self.n, self.x);
		if n == 0 { return None; }
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
