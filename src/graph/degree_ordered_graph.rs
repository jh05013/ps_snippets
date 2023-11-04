pub mod degree_ordered {
	type EdgeIndex = usize;

	/// A constructor for [`DegreeOrderedGraph`].
	#[derive(Debug, Clone, Default)]
	pub struct DogConstructor {
		adj: Vec<Vec<(usize, EdgeIndex)>>,
		edge_cnt: usize,
	}

	impl DogConstructor {
		/// Returns a new DOG constructor of `n` vertices.
		pub fn new(n: usize) -> Self {
			Self { adj: vec![vec![]; n], edge_cnt: 0 }
		}

		/// Connects vertices `a` and `b`.
		pub fn connect(&mut self, a: usize, b: usize) {
			let e = self.edge_cnt;
			self.adj[a].push((b, e));
			self.adj[b].push((a, e));
			self.edge_cnt += 1;
		}

		/// Turns the DOG constructor into [`DegreeOrderedGraph`].
		/// 
		/// The graph must be simple. Otherwise the time complexity
		/// will not be guaranteed.
		pub fn init(mut self) -> DegreeOrderedGraph {
			let n = self.adj.len();
			let degs = (0..n).map(|v| self.adj[v].len()).collect::<Vec<_>>();
			for row in &mut self.adj {
				row.sort_by_key(|&(v, _)| (degs[v], v));
			}
			DegreeOrderedGraph { adj: self.adj, edge_cnt: self.edge_cnt }
		}
	}

	/// The DOG interface. To construct it, see [`DogConstructor`].
	#[derive(Debug, Clone)]
	pub struct DegreeOrderedGraph {
		pub adj: Vec<Vec<(usize, EdgeIndex)>>,
		pub edge_cnt: usize,
	}

	impl DegreeOrderedGraph {
		/// Returns the number of vertices.
		pub fn len(&self) -> usize { self.adj.len() }
		/// Returns true iff empty.
		pub fn is_empty(&self) -> bool { self.adj.is_empty() }

		/// Returns the degree of vertex `v`.
		pub fn degree(&self, v: usize) -> usize { self.adj[v].len() }
		fn rank(&self, v: usize) -> (usize, usize) { (self.degree(v), v) }
		/// Returns true iff vertex `a` is ordered `b` in the graph,
		/// meaning (degree of `a`, `a`) < (degree of `b`, `b`).
		pub fn is_before(&self, a: usize, b: usize) -> bool { self.rank(a) < self.rank(b) }

		/// Iterates over the neighbors of vertex `v` that are ordered
		/// before vertex `pivot`, along with the edge indices.
		pub fn adj_before(&self, v: usize, pivot: usize)
		-> impl Iterator<Item = &(usize, EdgeIndex)> {
			self.adj[v].iter().take_while(move |(u, _)| self.is_before(*u, pivot))
		}

		/// Returns the number of quadrilaterals that pass through
		/// each vertex.
		/// 
		/// *Source: Paul Burkhardt, David G. Harris.*
		/// *Simple and efficient four-cycle counting on sparse graphs.*
		/// *[https://arxiv.org/abs/2303.06090](https://arxiv.org/abs/2303.06090)*
		pub fn quads_per_vertex(&self) -> Vec<u64> {
			let n = self.len();
			let mut ans = vec![0; n];
			let mut lp = vec![0; n];
			let mut l = vec![0; n];
			for v in 0..n {
				for &(u, _) in self.adj_before(v, v) {
					for &(y, _) in self.adj_before(u, v) {
						ans[v] += lp[y]; ans[y] += lp[y];
						lp[y] += 1; l[y] = lp[y];
					}
				}
				for &(u, _) in self.adj_before(v, v) {
					for &(y, _) in self.adj_before(u, v) {
						ans[u] += l[y]; ans[u] -= 1;
						lp[y] = 0;
					}
				}
			}
			ans
		}
	}
}
pub use degree_ordered::{DogConstructor, DegreeOrderedGraph};
