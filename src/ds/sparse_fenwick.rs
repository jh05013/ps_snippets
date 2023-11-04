pub mod dynamic_fenwick {
	use std::collections::HashMap;

	pub trait ComMonoid {
		type V: Copy;
		const ID: Self::V;
		fn op(lhs: Self::V, rhs: Self::V) -> Self::V;
	}

	pub trait InvOp: ComMonoid {
		fn inv_op(lhs: <Self as ComMonoid>::V, rhs: <Self as ComMonoid>::V) -> <Self as ComMonoid>::V;
	}

	type Coord = u32;
	pub struct DynamicFenwick<T: ComMonoid> {
		x: Coord,
		map: HashMap<Coord, T::V>,
	}

	impl<T: ComMonoid> DynamicFenwick<T> {
		pub fn new(x: Coord) -> Self {
			Self { x: x.next_power_of_two(), map: HashMap::new() }
		}

		pub fn add(&mut self, mut i: Coord, v: T::V) {
			assert!(self.x > i);
			while i < self.x {
				self.map.entry(i)
					.and_modify(|ent| *ent = T::op(*ent, v))
					.or_insert(v);
				i |= i+1;
			}
		}

		pub fn pref_sum(&self, mut i: Coord) -> T::V {
			let mut ans = T::ID;
			loop {
				ans = T::op(ans, *self.map.get(&i).unwrap_or(&T::ID));
				i &= i+1;
				if i == 0 { break; }
				i -= 1;
			}
			ans
		}
	}

	impl<T: ComMonoid + InvOp> DynamicFenwick<T> {
		pub fn sum(&self, l: Coord, r: Coord) -> T::V {
			assert!(l <= r, "Bad query range [{}, {}]", l, r);
			let vr = self.pref_sum(r);
			if l == 0 { vr } else { T::inv_op(vr, self.pref_sum(l-1)) }
		}
	}
}
pub use dynamic_fenwick::{ComMonoid, InvOp, DynamicFenwick};
