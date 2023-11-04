pub mod conncomp {
	pub fn connected_components(graph: &[Vec<usize>]) -> Vec<Vec<usize>> {
		let n = graph.len();
		let mut comps: Vec<Vec<usize>> = vec![];
		let mut vis: Vec<bool> = vec![false; n];
		for v in 0..n {
			if vis[v] { continue; }
			let mut comp = vec![v];
			let mut stack = vec![v];
			vis[v] = true;
			while let Some(p) = stack.pop() {
				for &q in &graph[p] { if !vis[q] {
					vis[q] = true; comp.push(q); stack.push(q);
				}}
			}
			comps.push(comp);
		}
		comps
	}
}
pub use conncomp::connected_components;
