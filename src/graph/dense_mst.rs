mod dense_mst {
	#[derive(Default, Clone, Debug)]
	pub struct Mst<T> {
		pub is_connected: bool,
		pub cost: T,
		pub edges: Vec<(usize, usize)>,
	}

	pub fn dense_mst<F, T>(n: usize, cost: F) -> Mst<T>
	where F: Fn(usize, usize) -> Option<T>,
	T: Ord + Default + std::ops::AddAssign {
		assert!(n > 0);

		let cost_cmp = |a: &Option<T>, b: &Option<T>| -> std::cmp::Ordering {
			if a.is_none() { a.cmp(b).reverse() } else { a.cmp(b) }
		};

		// cost, out of tree, inside tree
		let mut candidates = (1..n)
			.map(|v| (cost(0, v), v, 0usize))
			.collect::<Vec<_>>();
		let mut edges = vec![];
		let mut total_cost = T::default();

		for _ in 0..n-1 {
			let (i, chosen) = candidates.iter_mut()
				.enumerate()
				.min_by(|a, b| cost_cmp(&a.1.0, &b.1.0))
				.unwrap();
			let chosen = std::mem::take(chosen);
			candidates.swap_remove(i);

			let (mincost, new_inn, inn) = chosen;
			if let Some(mincost) = mincost {
				total_cost += mincost;
				edges.push((new_inn, inn));
			}
			for (prev_cost, out, inn) in candidates.iter_mut() {
				let new_cost = cost(*out, new_inn);
				if cost_cmp(prev_cost, &new_cost).is_le() { continue; }
				*prev_cost = new_cost;
				*inn = new_inn;
			}
		}
		
		Mst {
			is_connected: edges.len()+1 == n,
			cost: total_cost,
			edges,
		}
	}
} pub use dense_mst::{Mst, dense_mst};
