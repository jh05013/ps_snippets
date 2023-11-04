pub mod dynamic_segtree {
	pub trait Monoid {
		type V: Copy;
		const ID: Self::V;
		fn op(lhs: Self::V, rhs: Self::V) -> Self::V;
	}

	#[derive(Clone, Debug, Default)]
	pub struct DynamicSegtree<T: Monoid, X> {
		val: T::V,
		l: X, r: X,
		left: Option<Box<DynamicSegtree<T, X>>>,
		right: Option<Box<DynamicSegtree<T, X>>>,
	}

	impl<T: Monoid, X> DynamicSegtree<T, X>
	where X: PartialOrd + std::fmt::Debug + Copy +
	std::ops::Add<Output=X> + std::ops::Div<Output=X> + From<u8> {
		pub fn new(l: X, r: X) -> Self {
			assert!(l <= r, "Bad init range [{:?}, {:?}]", l, r);
			Self { val: T::ID, l, r, left: None, right: None }
		}

		fn refresh(&mut self) {
			self.val = T::ID;
			if let Some(st) = &self.left { self.val = st.val; }
			if let Some(st) = &self.right { self.val = T::op(self.val, st.val); }
		}

		pub fn change(&mut self, i: X, v: T::V) {
			if self.l > i || self.r < i { return; }
			if self.l == self.r { self.val = v; return; }
			let m = (self.l + self.r) / 2.into();
			if i <= m {
				if self.left.is_none() {
					self.left = Some(Box::new(Self::new(self.l, m)));
				}
				self.left.as_mut().unwrap().change(i, v);
			}
			else {
				if self.right.is_none() {
					self.right = Some(Box::new(Self::new(m+1.into(), self.r)));
				}
				self.right.as_mut().unwrap().change(i, v);
			}
			self.refresh();
		}

		pub fn apply(&mut self, i: X, v: T::V) {
			if self.l > i || self.r < i { return; }
			self.val = T::op(self.val, v);
			if self.l == self.r { return; }
			let m = (self.l + self.r) / 2.into();
			if i <= m {
				if self.left.is_none() {
					self.left = Some(Box::new(Self::new(self.l, m)));
				}
				self.left.as_mut().unwrap().apply(i, v);
			}
			else {
				if self.right.is_none() {
					self.right = Some(Box::new(Self::new(m+1.into(), self.r)));
				}
				self.right.as_mut().unwrap().apply(i, v);
			}
		}

		pub fn query(&self, l: X, r: X) -> T::V {
			assert!(l <= r, "Bad query range [{:?}, {:?}]", l, r);
			if self.l > r || self.r < l { return T::ID; }
			if l <= self.l && self.r <= r { return self.val; }
			let mut ans = T::ID;
			if let Some(st) = &self.left { ans = st.query(l, r); }
			if let Some(st) = &self.right { ans = T::op(ans, st.query(l, r)); }
			ans
		}
	}
}
pub use dynamic_segtree::{Monoid, DynamicSegtree};
