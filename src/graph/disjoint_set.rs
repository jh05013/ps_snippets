mod disjoint_set {
	#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
	pub struct DisjointSet { par: Vec::<usize> }
	impl DisjointSet {
		pub fn new(n: usize) -> Self { DisjointSet { par: (0..n).collect() } }
		pub fn len(&self) -> usize { self.par.len() }

		pub fn union(&mut self, a: usize, b: usize) -> bool {
			let (ar, br) = (self.find(a), self.find(b));
			if ar == br { return false; }
			self.par[ar] = br; true
		}
		pub fn find(&mut self, mut i: usize) -> usize {
			while self.par[i] != i {
				self.par[i] = self.par[self.par[i]];
				i = self.par[i];
			}
			i
		}
		pub fn is_united(&mut self, a: usize, b: usize) -> bool {
			self.find(a) == self.find(b)
		}

		pub fn roots(&mut self) -> impl Iterator<Item = usize> + '_ {
			(0..self.len()).filter(|v| self.find(*v) == *v)
		}
	}

	#[derive(Default, Clone, Debug)]
	pub struct MstSolver<T> {
		// cost, a, b, id
		edges: Vec<(T, usize, usize, usize)>
	}
	#[derive(Default, Clone, Debug)]
	pub struct Mst<T> {
		pub is_connected: bool,
		pub cost: T,
		pub edge_ids: Vec<usize>,
	}

	impl<T> MstSolver<T>
	where T: Default + Ord + Clone + std::ops::AddAssign {
		pub fn new() -> Self { Self { edges: vec![] } }
		pub fn vertex_count(&self) -> usize {
			self.edges.iter().map(|(_,a,b,_)| a.max(b)+1)
				.max().unwrap_or(0)
		}
		pub fn edge_count(&self) -> usize { self.edges.len() }
		pub fn is_empty(&self) -> bool { self.edges.is_empty() }
		pub fn connect(&mut self, a: usize, b: usize, cost: T) {
			self.edges.push((cost, a, b, self.edge_count()))
		}

		pub fn destruct(mut self) -> Vec<(T, usize, usize, usize)> {
			self.edges.sort_unstable_by_key(|(_,_,_,id)| *id);
			self.edges
		}

		pub fn solve(&mut self) -> Mst<T> {
			self.edges.sort_unstable();
			let n = self.vertex_count();
			let mut ds = DisjointSet::new(n);
			let mut total_cost = T::default();
			let mut tree = vec![];
			for (cost, a, b, id) in &self.edges {
				if !ds.union(*a, *b) { continue; }
				total_cost += cost.clone();
				tree.push(*id);
			}
			
			Mst {
				is_connected: n == 0 || tree.len()+1 == n,
				cost: total_cost,
				edge_ids: tree,
			}
		}
	}
} pub use disjoint_set::{DisjointSet, MstSolver, Mst};
