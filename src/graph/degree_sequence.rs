pub mod degree_sequence {
	pub fn is_degree_sequence(mut deg: Vec<u32>) -> bool {
		deg.sort(); deg.reverse();
		let mut dsum = 0;
		let n = deg.len() as u64;
		for k in 0..n {
			dsum += deg[k as usize] as u64;
			let rhs = k*(k+1) +
				(k+1..n).map(|i|
					std::cmp::min(deg[i as usize] as u64, k+1)
				).sum::<u64>();
			if dsum > rhs { return false; }
		}
		dsum % 2 == 0
	}

	pub fn graph_from_degree_sequence(deg: Vec<u32>)
	-> Option<Vec<Vec<usize>>> {
		let mut deg = deg.into_iter().enumerate()
			.map(|(v, d)| (d, v))
			.collect::<Vec<_>>();
		deg.sort();
		let n = deg.len();
		let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
		while let Some((d, p)) = deg.pop() {
			let dl = deg.len() as u32;
			if d > dl { return None; }
			for (dq, q) in &mut deg[(dl-d) as usize..] {
				if *dq == 0 { return None; }
				graph[p].push(*q); graph[*q].push(p);
				*dq -= 1;
			}
			deg.sort();
		}
		Some(graph)
	}
}
pub use degree_sequence::{is_degree_sequence, graph_from_degree_sequence};
