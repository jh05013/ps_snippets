pub mod segtree_lazy_mod {
	/// `T` must implement this trait for [`LazySegtree`].
	pub trait LazyMonoid {
		type V: Copy;
		type Lazy: Copy + PartialEq;
		/// Identity of the value operation.
		const ID: Self::V;
		/// Identity of the lazy operation.
		const LAZY_ID: Self::Lazy;
		/// The value operation.
		fn op(lhs: Self::V, rhs: Self::V) -> Self::V;
		/// The lazy operation.
		fn op_lazy(cur: Self::Lazy, up: Self::Lazy) -> Self::Lazy;
		/// Applies the lazy into the value.
		fn unlazy(v: Self::V, size: usize, lz: Self::Lazy) -> Self::V;
	}

	/// The lazy segment tree.
	#[derive(Clone, Debug, Default)]
	pub struct SegtreeLazy<T: LazyMonoid> {
		n: usize,
		arr: Vec<T::V>,
		lazy: Vec<T::Lazy>,
	}

	impl<T: LazyMonoid> SegtreeLazy<T> {
		/// Constructs a new lazy segment tree of length `n`.
		pub fn new(n: usize) -> Self {
			let sz = n.next_power_of_two();
			Self { n: sz, arr: vec![T::ID; sz*2], lazy: vec![T::LAZY_ID; sz*2] }
		}
		/// Constructs a new lazy segment tree out of `vals`.
		pub fn from_slice(vals: &[T::V]) -> Self {
			let sz = vals.len().next_power_of_two();
			let mut arr = vec![T::ID; sz*2];
			arr[sz..(vals.len() + sz)].copy_from_slice(vals);
			for i in (1..sz).rev() { arr[i] = T::op(arr[i*2], arr[i*2+1]); }
			Self { n: sz, arr, lazy: vec![T::LAZY_ID; sz*2] }
		}

		/// Applies the lazy operation of `lz` to the indices `l..=r`.
		pub fn update(&mut self, l: usize, r: usize, lz: T::Lazy) {
			assert!(l <= r && r < self.n, "Bad update range [{}, {}]",l,r);
			self.update_inner(l, r, lz, 1, 0, self.n-1);
		}
		/// Returns the result of the value operation over the indices `l..=r`.
		pub fn query(&mut self, l: usize, r: usize) -> T::V {
			assert!(l <= r && r < self.n, "Bad query range [{}, {}]",l,r);
			self.query_inner(l, r, 1, 0, self.n-1)
		}

		fn propagate(&mut self, i: usize, nl: usize, nr: usize) {
			if self.lazy[i] == T::LAZY_ID { return; }
			if i < self.n {
				self.lazy[i*2] = T::op_lazy(self.lazy[i*2], self.lazy[i]);
				self.lazy[i*2+1] = T::op_lazy(self.lazy[i*2+1], self.lazy[i]);
			}
			self.arr[i] = T::unlazy(self.arr[i], nr-nl+1, self.lazy[i]);
			self.lazy[i] = T::LAZY_ID;
		}

		fn update_inner(&mut self, l: usize, r: usize, val: T::Lazy,
		x: usize, nl: usize, nr: usize) {
			self.propagate(x, nl, nr);
			if r < nl || nr < l { return; }
			if l <= nl && nr <= r {
				self.lazy[x] = val;
				self.propagate(x, nl, nr);
				return;
			}
			let mid = (nl + nr) / 2;
			self.update_inner(l, r, val, x*2, nl, mid);
			self.update_inner(l, r, val, x*2+1, mid+1, nr);
			self.arr[x] = T::op(self.arr[x*2], self.arr[x*2+1]);
		}

		fn query_inner(&mut self, l: usize, r: usize,
		x: usize, nl: usize, nr: usize) -> T::V {
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
pub use segtree_lazy_mod::{LazyMonoid, SegtreeLazy};
