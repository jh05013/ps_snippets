pub mod dirichlet_convolution {
	use std::collections::HashMap;
	use std::ops::*;
	use super::div_floors;

	pub fn multiplicative_partial_sum<T>(
		f_small_sum: impl Fn(u64) -> Option<T>,
		g_sum: impl Fn(u64) -> T,
		fg_sum: impl Fn(u64) -> T,
		n: u64
	) -> HashMap<u64, T> where
	T: Clone + Default + SubAssign + Mul<Output=T> + Div<Output=T>
	{
		let mut f_sum_map = HashMap::<u64, T>::new();
		f_sum_map.insert(0, T::default());
		if n == 0 { return f_sum_map; }

		let mut sub_ns = div_floors(n).map(|(x,_,_)| x).collect::<Vec<_>>();
		sub_ns.reverse();

		for sub_n in sub_ns {
			if let Some(dp_val) = f_small_sum(sub_n) {
				f_sum_map.insert(sub_n, dp_val);
				continue;
			}

			let mut val = fg_sum(sub_n);
			for (sub_sub_n, l, r) in div_floors(sub_n).skip(1) {
				let sfx = f_sum_map[&sub_sub_n].clone();
				let mut glr = g_sum(r);
				glr -= g_sum(l-1);
				val -= sfx * glr;
			}
			f_sum_map.insert(sub_n, val / g_sum(1));
		}
		f_sum_map
	}
} pub use dirichlet_convolution::multiplicative_partial_sum;
