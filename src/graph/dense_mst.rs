mod dense_mst {
	#[derive(Default, Clone, Debug)]
	pub struct Mst<T> {
		pub is_connected: bool,
		pub total_cost: T,
		pub edges: Vec<(T, usize, usize)>,
	}

	pub fn dense_mst<F, T>(n: usize, cost: F) -> Mst<T>
	where F: Fn(usize, usize) -> Option<T>,
	T: Ord + Clone + Default + std::ops::AddAssign {
		// treat None as infinity
		let cost_cmp = |a: &Option<T>, b: &Option<T>| -> std::cmp::Ordering {
			if a.is_none() || b.is_none() { b.cmp(a) } else { a.cmp(b) }
		};

		// cost, vertex out of tree, vertex inside tree
		let mut candidates: Vec<_> = (1..n).map(|v| (cost(0, v), v, 0usize)).collect();
		let mut edges = vec![];
		let mut total_cost = T::default();

		for _ in 1..n {
			let (i, chosen) = candidates.iter_mut().enumerate()
				.min_by(|a, b| cost_cmp(&a.1.0, &b.1.0)).unwrap();
			let chosen = std::mem::take(chosen);
			candidates.swap_remove(i);

			let (mincost, new_inn, inn) = chosen;
			if let Some(mincost) = mincost {
				total_cost += mincost.clone();
				edges.push((mincost, new_inn, inn));
			}
			for (prev_cost, out, inn) in &mut candidates {
				let new_cost = cost((*out).min(new_inn), (*out).max(new_inn));
				if cost_cmp(prev_cost, &new_cost).is_le() { continue; }
				*prev_cost = new_cost; *inn = new_inn;
			}
		}
		
		Mst {
			is_connected: edges.len()+1 == n || n == 0,
			total_cost, edges,
		}
	}
} pub use dense_mst::{Mst, dense_mst};
