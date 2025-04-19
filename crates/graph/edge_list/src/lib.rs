//! A graph structure based on linked lists of edges.
//! 
//! Inspired from
//! [kiwiyou's snippet](https://snippets.kiwiyou.dev/graph).
//! See also
//! [this Codeforces blog](https://codeforces.com/blog/entry/67883).
//! 
//! # Example
//! ```
//! # use edge_list::Graph;
//! let mut graph = Graph::<i32>::new(5);
//! graph.direct(0, 3, -10); // 0 --> 3 with weight -10
//! graph.bidirect(1, 3, 8); // 1 <-> 3 with weight 8
//! assert_eq!(graph.vertex_count(), 5);
//! assert_eq!(graph.edge_count(), 3); // 1 <-> 3 counts twice
//! 
//! let edge = graph.edges_from(3).next().unwrap();
//! assert_eq!(edge.to, 1);
//! assert_eq!(edge.content, 8);
//! let v = graph.neighbors(1).next().unwrap();
//! assert_eq!(v, 3);
//! ```

use std::convert::TryFrom;

/// A (directed) edge of [`Graph`].
/// 
/// The start vertex is not stored here, and must
/// be retrieved from the graph if necessary.
/// The end vertex is stored as `to`.
/// 
/// An edge additionally has `content`, the "weight".
/// For unweighted graphs, use `()`.
#[derive(Clone, Debug, Default)]
pub struct Edge<T> {
	/// The end vertex.
	pub to: u32,
	/// The weight.
	pub content: T,
	prev: u32,
}

/// A graph with [`Edge`]s.
/// 
/// Inherently, the graph is assumed directed.
/// To make an undirected graph, remember to connect
/// in both directions (or use `bidirect`).
/// 
/// In the method descriptions,
/// let `n` be the number of vertices.
#[derive(Clone, Debug, Default)]
pub struct Graph<T> {
	heads: Vec<u32>,
	edges: Vec<Edge<T>>,
}

impl<T> Graph<T> {
	/// Returns an edge-less graph with `n` vertices.
	/// 
	/// ⚠️ Panics if `n >= u32::MAX`. (That's 4e9 vertices!)
	/// It turns out that using `u32` instead of `usize`
	/// for vertex indices leads to non-trivial speedup...
	#[inline]
	pub fn new(n: usize) -> Self {
		assert!(n < u32::MAX as usize,
			"why do you need {} vertices?!",
			n
		);
		Self { heads: vec![u32::MAX; n], edges: vec![] }
	}

	/// Returns `n`.
	#[inline]
	pub fn vertex_count(&self) -> usize {
		self.heads.len()
	}

	/// Returns the number of edges.
	#[inline]
	pub fn edge_count(&self) -> usize {
		self.edges.len()
	}

	#[inline]
	fn verify_vertex(&self, v: usize) {
		debug_assert!(
			v < self.vertex_count(),
			"invalid vertex {} on {} vertices",
			v,
			self.vertex_count()
		);
	}

	/// Adds a directed edge from `from` to `to`
	/// with weight `content`.
	/// 
	/// For unweighted graphs, simply let `T = ()`.
	/// 
	/// ⚠️ Panics if `from >= n` or `to >= n`.
	#[inline]
	pub fn direct(&mut self, from: usize, to: usize, content: T) -> usize {
		self.verify_vertex(from);
		self.verify_vertex(to);
		let index = u32::try_from(self.edges.len()).unwrap();

		let prev = std::mem::replace(&mut self.heads[from], index);
		self.edges.push(Edge { to: to as u32, content, prev });
		index as usize
	}

	/// Adds a direct edge from `a` to `b`,
	/// and from `b` to `a`, both with weight `content`.
	/// 
	/// For unweighted graphs, simply let `T = ()`.
	/// 
	/// ⚠️ Panics if `a >= n` or `b >= n`.
	#[inline]
	pub fn bidirect(&mut self, a: usize, b: usize, content: T) -> usize
	where T: Clone {
		let index = self.direct(a, b, content.clone());
		self.direct(b, a, content);
		index
	}

	/// Returns the list of edges.
	#[inline]
	pub fn edges(&self) -> &[Edge<T>] {
		&self.edges
	}
	
	/// Returns the edges starting from the vertex `v`.
	/// 
	/// ⚠️ Panics if `v >= n`.
	#[inline]
	pub fn edges_from(&self, v: usize) -> impl Iterator<Item = &Edge<T>> {
		std::iter::successors(
			self.edges.get(self.heads[v] as usize),
				move |edge| {
				self.edges.get(edge.prev as usize)
			}
		)
	}

	/// Returns the neighboring vertices of `v`.
	/// 
	/// If there are parallel edges, neighbors may appear
	/// more than once. If there are loops, `v` itself may appear.
	/// 
	/// ⚠️ Panics if `v >= n`.
	#[inline]
	pub fn neighbors(&self, v: usize) -> impl Iterator<Item = usize> + '_ {
		self.edges_from(v).map(|edge| edge.to as usize)
	}
}

pub type UnweightedGraph = Graph<()>;
