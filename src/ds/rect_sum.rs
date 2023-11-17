pub mod rect_sum {
	use super::fenwick::*;

	#[derive(Clone, Debug, Copy)]
	pub struct Rect<X> { pub l: X, pub d: X, pub r: X, pub u: X }

	pub fn static_rect_sums<T: ComMonoid + InvOp, X>
		(points: Vec<(X, X, T::V)>, rects: Vec<Rect<X>>) -> Vec<T::V>
	where T::V: PartialOrd, X: Copy + PartialOrd + std::fmt::Debug {
		// Edge case
		if points.is_empty() { return vec![T::ID; rects.len()]; }
		if rects.is_empty() { return vec![]; }

		#[derive(PartialEq, PartialOrd)]
		enum Event<V: PartialOrd, X> {
			Subtract(X, X, usize), // l, r, qidx
			Point(X, V), // x, val
			Add(X, X, usize), // l, r, qidx
		}

		// Construct sweeping events and compress x-coords
		let mut xs: Vec<X> = vec![];
		let mut events: Vec<(X, Event<T::V, X>)> = vec![];
		for (x, y, val) in points {
			xs.push(x);
			events.push((y, Event::Point(x, val)));
		}
		for (i, &rect) in rects.iter().enumerate() {
			assert!(rect.l <= rect.r, "Bad rectangle [l={:?}, r={:?}]", rect.l, rect.r);
			assert!(rect.d <= rect.u, "Bad rectangle [d={:?}, u={:?}]", rect.d, rect.u);
			events.push((rect.d, Event::Subtract(rect.l, rect.r, i)));
			events.push((rect.u, Event::Add(rect.l, rect.r, i)));
		}
		xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
		xs.dedup_by(|a, b| a==b);
		events.sort_by(|a,b| a.partial_cmp(b).unwrap());

		// Sweep
		let mut fen = Fenwick::<T>::new(xs.len());
		let mut ans = vec![T::ID; rects.len()];
		for (_, ev) in events {
			match ev {
				Event::Subtract(l, r, qidx) => {
					if xs[0] > r { continue; }
					let l = xs.partition_point(|x| x < &l);
					let r = xs.partition_point(|x| x <= &r)-1;
					if l <= r { ans[qidx] = T::inv_op(T::ID, fen.sum(l, r)); }
				}
				Event::Point(x, val) => {
					let x = xs.partition_point(|y| y < &x);
					fen.add(x, val);
				}
				Event::Add(l, r, qidx) => {
					if xs[0] > r { continue; }
					let l = xs.partition_point(|x| x < &l);
					let r = xs.partition_point(|x| x <= &r)-1;
					if l <= r { ans[qidx] = T::op(ans[qidx], fen.sum(l, r)); }
				}
			}
		}
		ans
	}

	#[derive(Clone, Debug, Copy)]
	pub enum DyrsEvent<T: ComMonoid + InvOp, X> {
		Add(X, X, T::V),
		Rect(Rect<X>),
	}

	pub fn dynamic_rect_sums<T: ComMonoid + InvOp, X>
		(events: &[DyrsEvent<T, X>]) -> Vec<T::V>
	where T::V: PartialOrd, X: Copy + PartialOrd + std::fmt::Debug {
		let n = events.len();

		// Base
		if n <= 1 {
			if n == 0 || matches!(events[0], DyrsEvent::Add(..)) { return vec![]; }
			else { return vec![T::ID]; }
		}

		// Divide
		let h = n/2;
		let mut ans_left = dynamic_rect_sums(&events[..h]);
		let mut ans_right = dynamic_rect_sums(&events[h..]);

		// Combine
		let cross_points = events[..h].iter()
			.filter_map(|ev| match ev {
				DyrsEvent::Add(x, y, v) => Some((*x, *y, *v)), _ => None
			})
			.collect::<Vec<_>>();
		let cross_rects = events[h..].iter()
			.filter_map(|ev| match ev {
				DyrsEvent::Rect(r) => Some(*r), _ => None
			})
			.collect::<Vec<_>>();
		for (z, add) in ans_right.iter_mut().zip(
			static_rect_sums::<T, _>(cross_points, cross_rects).iter()
		) { *z = T::op(*z, *add); }
		ans_left.append(&mut ans_right);
		ans_left
	}
}
pub use rect_sum::{Rect, static_rect_sums, DyrsEvent, dynamic_rect_sums};
