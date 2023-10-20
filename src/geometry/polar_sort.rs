pub mod polar_sort {
	use super::geom_point::*;

	fn half<T: Coord>(p: Pnt<T>) -> bool {
		assert!(p != Pnt::<T>::default(), "Cannot polar-sort the origin");
		p.y > T::default() || (p.y == T::default() && p.x < T::default())
	}

	pub fn polar_sort<T: Coord + Ord>(pnts: &mut Vec<Pnt<T>>) {
		pnts.sort_by(|&p, &q| {
			(half(p), T::default(), p.sq()).cmp( &(half(q), p.cross(q), q.sq()) )
		});
	}
	pub fn polar_sort_at<T: Coord + Ord>(pnts: &mut Vec<Pnt<T>>, c: Pnt<T>) {
		for p in pnts.iter_mut() { *p -= c; }
		polar_sort(pnts);
		for p in pnts { *p += c; }
	}

	pub fn polar_sorted_halfplanes<T: Coord + Ord>(pnts: &[Pnt<T>]) -> Vec<usize> {
		let n = pnts.len();
		let o = Pnt::<T>::default();
		let mut ri = 0;
		let mut ans = vec![];
		for li in 0..n {
			while ri < li+n &&
				pnts[li].orient(o, pnts[ri%n]) <= T::default() { ri += 1; }
			ans.push(ri-1);
		}
		ans
	}
}
pub use polar_sort::*;
