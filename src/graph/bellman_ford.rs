pub mod bellman_ford {
	#[derive(Clone, Debug, Default)]
	pub struct BellmanFord<T> {
		edges: Vec<(T, usize, usize)>,
	}

	impl<T> BellmanFord<T>
	where T: Clone + Default + PartialOrd + std::ops::Add<Output=T> {
		pub fn new() -> Self { Self { edges: vec![] } }
		pub fn vertex_count(&self) -> usize {
			self.edges.iter().map(|(_,a,b)| a.max(b)+1)
				.max().unwrap_or(0)
		}
		pub fn edge_count(&self) -> usize { self.edges.len() }
		pub fn is_empty(&self) -> bool { self.edges.is_empty() }

		pub fn connect(&mut self, fr: usize, to: usize, cost: T) {
			self.edges.push((cost, fr, to));
		}
		/// Registers the inequality `x_i - x_j <= d`.
		pub fn difference_ineq(&mut self, i: usize, j: usize, d: T) {
			self.connect(j, i, d);
		}

		pub fn solve(&self, source: usize) -> Result<Vec<Option<T>>, ()> {
			let n = self.vertex_count();
			let mut dist: Vec<Option<T>> = vec![None; n];
			dist[source] = Some(T::default());

			let mut update_edge = |(c, a, b): &(T, usize, usize)| -> bool {
				let Some(da) = &dist[*a] else { return false };
				let newc = da.clone() + c.clone();
				let Some(db) = &dist[*b] else { dist[*b] = Some(newc); return true; };
				if &newc < db { dist[*b] = Some(newc); return true; }
				else { return false; }
			};

			for _ in 1..n { for edge in &self.edges {
				update_edge(edge);
			}}
			for edge in &self.edges {
				if update_edge(edge) { return Err(()); }
			}
			Ok(dist)
		}
	}
} pub use bellman_ford::BellmanFord;
