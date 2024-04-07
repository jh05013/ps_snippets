
pub mod fenwick {
	/// The trait that enables prefix sum on Fenwick trees.
	pub trait FenwickOp {
		type V: Clone;
		const ID: Self::V;
		fn add(cur: &mut Self::V, val: &Self::V);
	}

	/// The trait that enables arbitrary range sum on Fenwick trees.
	pub trait InvOp: FenwickOp {
		fn sub(cur: &mut <Self as FenwickOp>::V, val: &<Self as FenwickOp>::V);
	}

	/// Fenwick tree.
	#[derive(Clone, Debug, Default, PartialEq, Eq)]
	pub struct Fenwick<T: FenwickOp>(Vec<T::V>);

	impl<T: FenwickOp> From<Vec<T::V>> for Fenwick<T> {
		fn from(mut value: Vec<T::V>) -> Self {
			for i in 0..value.len() {
				// SAFETY: i < value.len()
				let v = unsafe { value.get_unchecked(i) }.clone();
				if let Some(cur) = value.get_mut(i|(i+1)) { T::add(cur, &v); }
			}
			Self(value)
		}
	}

	impl<T: FenwickOp> Fenwick<T> {
		/// Initializes an empty Fenwick tree of size `n`.
		pub fn new(n: usize) -> Self { Self(vec![T::ID; n]) }
		/// Returns the length of the Fenwick tree.
		pub fn len(&self) -> usize { self.0.len() }

		/// Adds `v` at index `i`.
		pub fn add(&mut self, mut i: usize, v: &T::V) {
			assert!(i < self.len(), "Bad update index {}", i);
			while let Some(cur) = self.0.get_mut(i) {
				T::add(cur, v); i |= i+1;
			}
		}

		/// Sums up to index `i`.
		pub fn prefix_sum(&self, mut i: usize) -> T::V {
			assert!(i < self.len(), "Bad query range [..={}]", i);
			let mut ans = T::ID;
			loop {
				// SAFETY: i < self.len() and i decreases
				let v = unsafe { self.0.get_unchecked(i) };
				T::add(&mut ans, v);
				i &= i+1;
				i = if i != 0 { i-1 } else { return ans };
			}
		}

		/// Binary search on the values of prefix sums,
		/// see [`slice::partition_point`].
		pub fn partition_point<P>(&self, pred: P) -> usize
		where P: Fn(&T::V) -> bool {
			let mut i = 0;
			let mut step = self.len().next_power_of_two() >> 1;
			let mut left = T::ID;
			while step > 0 {
				let Some(val) = self.0.get(i + step-1) else {
					step >>= 1; continue;
				};
				let mut sum = left.clone();
				T::add(&mut sum, val);
				if pred(&sum) { left = sum; i += step; }
				step >>= 1;
			}
			i
		}
	}

	impl<T: FenwickOp + InvOp> Fenwick<T> {
		/// Sums from index `l` to `r`.
		pub fn range_sum(&self, l: usize, r: usize) -> T::V {
			assert!(l <= r, "Bad query range [{}, {}]", l, r);
			let mut vr = self.prefix_sum(r);
			if l != 0 { T::sub(&mut vr, &self.prefix_sum(l-1)); }
			vr
		}

		/// Subtracts `v` at index `i`.
		pub fn sub(&mut self, mut i: usize, v: &T::V) {
			assert!(i < self.len(), "Bad update index {}", i);
			while let Some(cur) = self.0.get_mut(i) {
				T::sub(cur, v); i |= i+1;
			}
		}
	}

	// Usual sum impl
	macro_rules! impl_fenwick_sum {
		($($T:ty) *) => { $(
			#[allow(clippy::as_conversions)]
			impl FenwickOp for $T {
				type V = $T; const ID: $T = 0 as $T;
				fn add(cur: &mut $T, val: &$T) { *cur+=*val; }
			}
			impl InvOp for $T {
				fn sub(cur: &mut $T, val: &$T) { *cur-=*val; }
			}
		)* };
	}
	impl_fenwick_sum!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
}
pub use fenwick::{FenwickOp, InvOp, Fenwick};
