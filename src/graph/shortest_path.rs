pub mod shortest_path {
	use std::collections::VecDeque;

	#[derive(Clone, Debug, Default)]
	pub struct ShortestPathTracer<T> {
		pub dist_prv: Vec<Option<(T, usize)>>,
	}
	
	impl<T> std::ops::Index<usize> for ShortestPathTracer<T> {
		type Output = Option<(T, usize)>;
		fn index(&self, index: usize) -> &Self::Output { &self.dist_prv[index] }
	}

	impl<T> ShortestPathTracer<T> {
		pub fn path_to(&self, mut sink: usize) -> Vec<usize> {
			let mut path = vec![sink];
			while let Some((_, v)) = self[sink] {
				if v == sink { break; }
				path.push(v); sink = v;
			}
			path.reverse();
			path
		}
	}

	/// BFS.
	pub fn bfs(graph: &[Vec<usize>], source: usize) -> ShortestPathTracer<usize> {
		let n = graph.len();
		let mut dist_prv = vec![None; n];
		dist_prv[source] = Some((0, source));
		let mut q = VecDeque::new();
		q.push_front(source);
		while let Some(fr) = q.pop_back() {
			let d = dist_prv[fr].unwrap().0;
			for &to in &graph[fr] { if dist_prv[to].is_none() {
				dist_prv[to] = Some((d+1, fr));
				q.push_front(to);
			}}
		}
		ShortestPathTracer { dist_prv }
	}
}
pub use shortest_path::{ShortestPathTracer, bfs};
