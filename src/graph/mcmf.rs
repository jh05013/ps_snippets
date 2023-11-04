// edited from https://bamgoesn.github.io/rust-ps-md/graphs/flow/mcmf.html
// TODO: support edge lookup
pub mod mcmf_mod {
	use std::collections::VecDeque;

	#[derive(Clone, Debug, Default)]
	pub struct Edge {
		pub dst: u32, pub opp: u32,
		pub cap: u64, pub cost: i64,
	}

	impl Edge {
		fn new(dst: usize, opp: usize, flow: u64, cost: i64) -> Self {
			Self { dst: dst as u32, opp: opp as u32, cap: flow, cost, }
		}
	}

	#[derive(Clone, Debug, Default)]
	pub struct Mcmf {
		pub n: usize,
		pub g: Vec<Vec<Edge>>,
	}

	impl Mcmf {
		pub fn new(n: usize) -> Self { Self { n, g: vec![vec![]; n], } }

		pub fn connect(&mut self, s: usize, e: usize, cap: u64, cost: i64) {
			let (slen, elen) = (self.g[s].len(), self.g[e].len());
			self.g[s].push(Edge::new(e, elen, cap, cost));
			self.g[e].push(Edge::new(s, slen, 0, -cost));
		}

		fn augment(&mut self, s: usize, t: usize,
		prv: &mut [usize], idx: &mut [usize], dst: &mut [i64]) -> bool {
			let mut inn = vec![false; self.n];
			dst.fill(i64::MAX);
			inn[s] = true; dst[s] = 0;
			let mut queue = VecDeque::new();
			queue.push_back(s);

			while let Some(v) = queue.pop_front() {
				inn[v] = false;
				for (i, e) in self.g[v].iter().enumerate() {
					let src = e.dst as usize;
					if e.cap != 0 && dst[src] > dst[v] + e.cost {
						dst[src] = dst[v] + e.cost;
						prv[src] = v; idx[src] = i;
						if !inn[src] { inn[src] = true; queue.push_back(src); }
					}
				}
			}
			dst[t] < i64::MAX
		}

		pub fn solve(&mut self, src: usize, sink: usize) -> (u64, i64) {
			use std::iter::successors;
			let mut flow = 0; let mut cost = 0;
			let mut prv = vec![0; self.n];
			let mut idx = vec![0; self.n];
			let mut dst = vec![0; self.n];
			while self.augment(src, sink, &mut prv, &mut idx, &mut dst) {
				let path = successors(Some(sink), |&i| Some(prv[i]))
					.take_while(|&i| i != src).map(|i| self.g[prv[i]][idx[i]].cap)
					.min().unwrap();
				flow += path; cost += path as i64 * dst[sink];
				for i in successors(Some(sink), |&i| Some(prv[i]))
				.take_while(|&i| i != src) {
					self.g[prv[i]][idx[i]].cap -= path;
					let j = self.g[prv[i]][idx[i]].opp as usize;
					self.g[i][j].cap += path;
				}
			}

			(flow, cost)
		}
	}
}
pub use mcmf_mod::Mcmf;
