pub mod lazy_segtree {
	use std::fmt::Debug;

	/// `T` must implement this trait for [`LazySegtree`].
	pub trait LazyMonoid: Copy + Debug {
		type Lazy: Copy + Debug + PartialEq;
		/// Identity of the value operation.
		const ID: Self;
		/// Identity of the lazy operation.
		const LAZY_ID: Self::Lazy;
		/// The value operation.
		fn op(a: Self, b: Self) -> Self;
		/// The lazy operation.
		fn op_lazy(a: Self::Lazy, b: Self::Lazy) -> Self::Lazy;
		/// Tries to apply the lazy into the value.
		/// If unsuccessful (Segment Tree Beats), returns [`None`].
		fn unlazy(v: Self, size: usize, lz: Self::Lazy) -> Option<Self>;
	}

	/// The lazy segment tree.
	#[derive(Clone, Debug, Default)]
	pub struct LazySegtree<T: LazyMonoid> {
		n: usize,
		arr: Vec<T>,
		lazy: Vec<T::Lazy>,
	}

	impl<T: LazyMonoid> LazySegtree<T> {
		/// Constructs a new lazy segment tree of length `n`.
		pub fn new(n: usize) -> Self {
			let sz = n.next_power_of_two();
			Self { n: sz, arr: vec![T::ID; sz*2], lazy: vec![T::LAZY_ID; sz*2] }
		}
		/// Constructs a new lazy segment tree out of `vals`.
		pub fn from_vec(vals: &[T]) -> Self {
			let sz = vals.len().next_power_of_two();
			let mut arr = vec![T::ID; sz*2];
			for i in 0..vals.len() { arr[i+sz] = vals[i]; }
			for i in (1..sz).rev() { arr[i] = T::op(arr[i*2], arr[i*2+1]); }
			Self { n: sz, arr, lazy: vec![T::LAZY_ID; sz*2] }
		}

		/// Applies the lazy operation of `val` to the indices `l..=r`.
		pub fn update(&mut self, l: usize, r: usize, val: T::Lazy) {
			assert!(l <= r && r < self.n, "Tried to update on {} {}",l,r);
			self.update_inner(l, r, val, 1, 0, self.n-1);
		}
		/// Returns the result of the value operation over the indices `l..=r`.
		pub fn query(&mut self, l: usize, r: usize) -> T {
			assert!(l <= r && r < self.n, "Tried to query {} {}",l,r);
			self.query_inner(l, r, 1, 0, self.n-1)
		}

		fn propagate(&mut self, i: usize, nl: usize, nr: usize) {
			if self.lazy[i] == T::LAZY_ID { return; }
			if i < self.n {
				self.lazy[i*2] = T::op_lazy(self.lazy[i*2], self.lazy[i]);
				self.lazy[i*2+1] = T::op_lazy(self.lazy[i*2+1], self.lazy[i]);
			}
			if let Some(v) = T::unlazy(self.arr[i], nr-nl+1, self.lazy[i]) {
				self.arr[i] = v;
				self.lazy[i] = T::LAZY_ID;
				return;
			}
			
			assert!(i < self.n, "Unlazy failed for leaf {:?} <- {:?}", self.arr[i], self.lazy[i]);
			let mid = (nl+nr)/2;
			self.propagate(i*2, nl, mid);
			self.propagate(i*2+1, mid+1, nr);
			self.arr[i] = T::op(self.arr[i*2], self.arr[i*2+1]);
			self.lazy[i] = T::LAZY_ID;
		}

		fn update_inner(&mut self, l: usize, r: usize, val: T::Lazy,
		x: usize, nl: usize, nr: usize) {
			self.propagate(x, nl, nr);
			if r < nl || nr < l { return; }
			if l <= nl && nr <= r {
				self.lazy[x] = T::op_lazy(self.lazy[x], val);
				self.propagate(x, nl, nr);
				return;
			}
			let mid = (nl + nr) / 2;
			self.update_inner(l, r, val, x*2, nl, mid);
			self.update_inner(l, r, val, x*2+1, mid+1, nr);
			self.arr[x] = T::op(self.arr[x*2], self.arr[x*2+1]);
		}

		fn query_inner(&mut self, l: usize, r: usize,
		x: usize, nl: usize, nr: usize) -> T {
			self.propagate(x, nl, nr);
			if r < nl || nr < l { return T::ID; }
			if l <= nl && nr <= r { return self.arr[x]; }
			let mid = (nl + nr) / 2;
			T::op(
				self.query_inner(l, r, x*2, nl, mid),
				self.query_inner(l, r, x*2+1, mid+1, nr)
			)
		}
	}
}
pub use lazy_segtree::{LazyMonoid, LazySegtree};
