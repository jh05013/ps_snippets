pub mod rectangle_union {
	use super::lazy_segtree::*;

	#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
	pub struct Rect{
		pub lo: i32, pub hi: i32,
		pub l: i32, pub r: i32
	}
	
	#[derive(Debug, Clone, Copy)]
	struct RuNode(i32, i32); // (minval, minwidth)

	impl LazyMonoid for RuNode {
		type Lazy = i32;
		const ID: Self = Self(i32::MAX, 0);
		const LAZY_ID: Self::Lazy = 0;
	
		fn op(a: Self, b: Self) -> Self {
			let Self(va, ca) = a;
			let Self(vb, cb) = b;
			if va == vb { Self(va, ca+cb) }
			else if va > vb { b } else { a }
		}
		fn op_lazy(a: Self::Lazy, b: Self::Lazy) -> Self::Lazy { a + b }
		fn unlazy(v: Self, _: usize, lz: Self::Lazy) -> Option<Self> {
			Some(Self(v.0+lz, v.1))
		}
	}

	pub fn rectangle_union(rects: Vec<Rect>) -> i64 {
		let mut xcoords = vec![];
		let mut events = vec![];
		for Rect{lo,hi,l,r} in rects {
			assert!(lo <= hi && l <= r, "Bad rectangle");
			if lo == hi || l == r { continue; }
			xcoords.push(l);
			xcoords.push(r);
			events.push((lo, 1, l, r));
			events.push((hi, -1, l, r));
		}
		events.sort();
		xcoords.sort(); xcoords.dedup();

		let mut st = LazySegtree::from_vec(
			&xcoords.windows(2)
				.map(|v| RuNode(0, v[1]-v[0]))
				.collect::<Vec<_>>()
		);
		let total_width = xcoords.last().unwrap() - xcoords[0];
		let node_cnt = xcoords.len() - 1;
		let mut ans = 0i64;
		for (i, &(y, ev, l, r)) in events.iter().enumerate() {
			if i != 0 {
				let dy = (y - events[i-1].0) as i64;
				let RuNode(mlay, mwidth) = st.query(0, node_cnt-1);
				ans+= dy * (total_width - if mlay == 0 { mwidth } else { 0 }) as i64;
			}
			let l = xcoords.binary_search(&l).unwrap();
			let r = xcoords.binary_search(&r).unwrap();
			st.update(l, r-1, ev);
		}
		ans
	}
}
pub use rectangle_union::{Rect, rectangle_union};
