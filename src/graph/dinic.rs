pub mod dinic_mod {
	#[derive(Clone, Debug)]
	pub struct Edge {
		pub src: usize, pub targ: usize,
		res: u64, pub orig: u64,
		rev: usize
	}
	#[derive(Copy, Clone, Debug)]
	pub struct EdgeIndex(usize, usize);

	impl Edge {
		pub fn used(&self) -> u64 { self.orig - self.res }
	}
	
	#[derive(Clone, Debug)]
	pub struct Dinic {
		pub n: usize,
		pub adj: Vec<Vec<Edge>>,
		dis: Vec<u32>,
		pnt: Vec<usize>
	}

	impl Dinic {
		/// Initializes a graph with `n` vertices.
		pub fn new(n: usize) -> Self { Self {
			n, adj: vec![vec![]; n],
			dis: vec![0; n], pnt: vec![0; n]
		}}

		/// Connects from `s` to `e` with capacity `cap`,
		/// and returns the edge index.
		pub fn connect(&mut self, s: usize, e: usize, cap: u64) -> EdgeIndex {
			let el = self.adj[e].len();
			let sl = self.adj[s].len();
			self.adj[s].push(Edge {
				src: s, targ: e, res: cap, orig: cap, rev: el
			});
			self.adj[e].push(Edge {
				src: e, targ: s, res: 0, orig: 0, rev: sl
			});
			EdgeIndex(s, sl)
		}

		/// Sends and returns the maximum flow from `src` to `sink`.
		pub fn max_flow(&mut self, src: usize, sink: usize) -> u64 {
			let mut ans = 0u64;
			while self.bfs(src, sink) {
				let mut r: u64;
				loop {
					r = self.dfs(src, sink, u64::MAX);
					if r == 0 { break; }
					ans += r;
				}
			}
			ans
		}

		/// Sends and returns the minimum cut between `src` and `sink`,
		/// along with the list of vertices in the `src` side.
		pub fn min_cut(&mut self, src: usize, sink: usize)
		-> (u64, Vec<usize>) {
			let f = self.max_flow(src, sink);
			self.bfs(src, sink);
			(f, (0..self.n).filter(|v| self.dis[*v] > 1).collect())
		}

		fn bfs(&mut self, src: usize, sink: usize) -> bool {
			self.dis.fill(0); self.pnt.fill(0);
			let mut q = std::collections::VecDeque::new();
			q.push_back(src); self.dis[src] = 1;
			while let Some(v) = q.pop_front() {
				for e in &self.adj[v] {
					if e.res > 0 && self.dis[e.targ] == 0 {
						self.dis[e.targ] = self.dis[v] + 1;
						q.push_back(e.targ);
					}
				}
			}
			self.dis[sink] > 0
		}
		fn dfs(&mut self, v: usize, sink: usize, f: u64) -> u64 {
			if v == sink { return f; }
			while self.pnt[v] < self.adj[v].len() {
				let e = self.adj[v][self.pnt[v]].clone();
				if e.res > 0 && self.dis[e.targ] == self.dis[v] + 1 {
					let w = self.dfs(e.targ, sink, f.min(e.res));
					if w == 0 { self.pnt[v] += 1; continue; }
					self.adj[v][self.pnt[v]].res -= w;
					self.adj[e.targ][e.rev].res += w;
					return w;
				}
				self.pnt[v] += 1;
			}
			0
		}
	}

	impl std::ops::Index<EdgeIndex> for Dinic {
		type Output = Edge;
		fn index(&self, ei: EdgeIndex) -> &Self::Output {
			&self.adj[ei.0][ei.1]
		}
	}
}
pub use dinic_mod::Dinic;
