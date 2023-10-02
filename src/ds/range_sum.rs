pub mod range_sum {
	use std::ops::*;

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	pub struct RangeSum<T> {
		pref: Vec<T>,
	}
	
	impl<T> RangeSum<T> where
	T: Clone + Default + AddAssign + SubAssign {
		/// Constructs a new range sum DS out of `vals`.
		pub fn new(mut vals: Vec<T>) -> Self {
			let n = vals.len();
			for i in 1..n { let v = vals[i-1].clone(); vals[i] += v; }
			Self { pref: vals }
		}
		/// Returns the length of `vals`.
		pub fn len(&self) -> usize { self.pref.len() }
		/// Returns whether `vals` is empty.
		pub fn is_empty(&self) -> bool { self.pref.is_empty() }
	
		/// Returns `vals[0] + ... + vals[i]`.
		pub fn pref_sum(&self, i: usize) -> T { self.pref[i].clone() }
		/// Returns the sum of all `vals[i]`, or 0 if empty.
		pub fn whole(&self) -> T {
			if self.is_empty() { T::default() } else { self.pref.last().unwrap().clone() }
		}
		/// Returns `vals[i] + ... + vals[j]`.
		pub fn sum(&self, i: usize, j: usize) -> T {
			let mut z = self.pref[j].clone();
			if i != 0 { z -= self.pref[i-1].clone(); }
			z
		}
	
		/// Pushes `v` at the end of `vals`.
		pub fn push(&mut self, v: T) {
			let mut z = self.whole(); z += v;
			self.pref.push(z)
		}
		/// Pushes `0` at the end of `vals` until its size is >= `n`.
		pub fn extend_to(&mut self, n: usize) {
			while self.len() < n { self.pref.push(self.whole()); }
		}
	}	
} pub use range_sum::RangeSum;
