pub mod degree_ordered {
	use std::ops::*;

	type EdgeIndex = usize;

	#[derive(Debug, Clone, Default)]
	pub struct DogConstructor {
		adj: Vec<Vec<(usize, EdgeIndex)>>,
		edge_cnt: usize,
	}

	impl DogConstructor {
		pub fn new(n: usize) -> Self {
			Self { adj: vec![vec![]; n], edge_cnt: 0 }
		}

		pub fn connect(&mut self, a: usize, b: usize) {
			let e = self.edge_cnt;
			self.adj[a].push((b, e));
			self.adj[b].push((a, e));
			self.edge_cnt += 1;
		}

		pub fn init(mut self) -> DegreeOrderedGraph {
			let n = self.adj.len();
			let degs = (0..n).map(|v| self.adj[v].len()).collect::<Vec<_>>();
			for row in &mut self.adj {
				row.sort_by_key(|&(v, _)| (degs[v], v));
			}
			DegreeOrderedGraph { adj: self.adj, edge_cnt: self.edge_cnt }
		}
	}

	#[derive(Debug, Clone)]
	pub struct DegreeOrderedGraph {
		pub adj: Vec<Vec<(usize, EdgeIndex)>>,
		pub edge_cnt: usize,
	}

	impl DegreeOrderedGraph {
		pub fn len(&self) -> usize { self.adj.len() }

		pub fn degree(&self, v: usize) -> usize { self.adj[v].len() }
		pub fn rank(&self, v: usize) -> (usize, usize) { (self.degree(v), v) }
		pub fn is_before(&self, a: usize, b: usize) -> bool { self.rank(a) < self.rank(b) }

		pub fn adj_before(&self, v: usize, pivot: usize)
		-> impl Iterator<Item = &(usize, EdgeIndex)> {
			self.adj[v].iter().take_while(move |(u, _)| self.is_before(*u, pivot))
		}

		pub fn quads_per_vertex<T>(&self) -> Vec<T>
		where T: Copy + Default + From<i32> + AddAssign + SubAssign {
			let n = self.len();
			let mut ans = vec![T::default(); n];
			let mut lp = vec![T::default(); n];
			let mut l = vec![T::default(); n];
			for v in 0..n {
				for &(u, _) in self.adj_before(v, v) {
					for &(y, _) in self.adj_before(u, v) {
						ans[v] += lp[y]; ans[y] += lp[y];
						lp[y] += 1.into(); l[y] = lp[y];
					}
				}
				for &(u, _) in self.adj_before(v, v) {
					for &(y, _) in self.adj_before(u, v) {
						ans[u] += l[y]; ans[u] -= 1.into();
						lp[y] = T::default();
					}
				}
			}
			ans
		}
	}
} use degree_ordered::*;
