pub mod dijkstra {
	use std::{collections::BinaryHeap, cmp::Reverse};

	#[derive(Clone, Debug, PartialEq)]
	pub enum EdgeOrient { Directed, Undirected }

	impl Default for EdgeOrient {
		fn default() -> Self { EdgeOrient::Directed }
	}

	#[derive(Clone, Debug, Default)]
	pub struct Dijkstra<T> {
		edge_orient: EdgeOrient,
		pub adj: Vec<Vec<(usize, T)>>,
		pub dist: Vec<Option<T>>,
		pub prv: Vec<Option<usize>>,
	}

	impl<T> Dijkstra<T>
	where T: Copy + Default + PartialEq + Ord + std::ops::Add<Output=T> {
		pub fn new(n: usize, edge_orient: EdgeOrient) -> Self {
			Self { edge_orient, adj: vec![vec![]; n], dist: vec![], prv: vec![] }
		}
		#[inline] pub fn n(&self) -> usize { self.adj.len() }
		#[inline] pub fn is_directed(&self) -> bool {
			self.edge_orient == EdgeOrient::Directed
		}
		#[inline] pub fn is_undirected(&self) -> bool {
			self.edge_orient == EdgeOrient::Undirected
		}

		pub fn connect(&mut self, a: usize, b: usize, cost: T) {
			self.adj[a].push((b, cost));
			if self.is_undirected() { self.adj[b].push((a, cost)); }
		}
		pub fn disconnect(&mut self, a: usize, b: usize) -> Option<T> {
			let Some(i) = self.adj[a].iter()
				.position(|&(v, _)| v == b) else { return None; };
			let (_, cost) = self.adj[a][i];
			self.adj[a].remove(i);
			if self.is_undirected() {
				let i = self.adj[b].iter()
					.position(|(v, c)| v == &a && c == &cost).unwrap();
				self.adj[b].remove(i);
			}
			Some(cost)
		}

		pub fn solve(&mut self, start: usize) -> &Vec<Option<T>> {
			self.dist = vec![None; self.n()];
			self.prv = vec![None; self.n()];
			let mut pq = BinaryHeap::<Reverse<(T, usize)>>::new();
			pq.push(Reverse((T::default(), start)));
			self.dist[start] = Some(T::default());
			while let Some(Reverse((d, v))) = pq.pop() {
				if self.dist[v] != Some(d) { continue; }
				for &(u, c) in &self.adj[v] {
					let nd = d + c;
					if let Some(pd) = self.dist[u] { if pd <= nd { continue; }}
					self.dist[u] = Some(nd);
					self.prv[u] = Some(v);
					pq.push(Reverse((nd, u)));
				}
			}
			&self.dist
		}

		pub fn path(&self, end: usize) -> Vec<usize> {
			let mut path = vec![end];
			loop {
				let v = *path.last().unwrap();
				let Some(u) = self.prv[v] else { path.reverse(); return path; };
				path.push(u);
			}
		}
	}
}
pub use dijkstra::{EdgeOrient, Dijkstra};
