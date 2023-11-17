pub mod range_sum_mod {
	use std::ops::*;

	#[derive(Clone, Debug, Default, Eq, PartialEq)]
	pub struct RangeSum<T> {
		pref: Vec<T>,
	}
	
	impl<T> RangeSum<T> where
	T: Clone + Default + AddAssign + Sub<Output=T> {
		/// Constructs an empty range sum DS.
		pub fn new() -> Self { Self::default() }
		/// Constructs a new range sum DS out of `vals`.
		pub fn from_vec(mut vals: Vec<T>) -> Self {
			let n = vals.len();
			for i in 1..n { let v = vals[i-1].clone(); vals[i] += v; }
			Self { pref: vals }
		}
		/// Returns the length of `vals`.
		pub fn len(&self) -> usize { self.pref.len() }
		/// Returns whether `vals` is empty.
		pub fn is_empty(&self) -> bool { self.pref.is_empty() }

		/// Returns `vals[i] + ... + vals[j]`.
		pub fn sum(&self, i: usize, j: usize) -> T {
			assert!(i <= j, "Bad query range [{}, {}]", i, j);
			if i == 0 { self.pref[j].clone() }
			else { self.pref[j].clone() - self.pref[i-1].clone() }
		}
		/// Returns the sum of all `vals[i]`, or 0 if empty.
		pub fn whole(&self) -> T {
			if self.is_empty() { T::default() } else { self.pref.last().unwrap().clone() }
		}
	
		/// Pushes `v` at the end of `vals`.
		pub fn push(&mut self, v: T) {
			let mut z = self.whole(); z += v;
			self.pref.push(z);
		}
		/// Truncates or extends (with 0) `vals` to length `n`.
		pub fn resize(&mut self, n: usize) {
			self.pref.resize(n, self.whole());
		}
	}	
}
pub use range_sum_mod::RangeSum;
