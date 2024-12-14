//! Dijkstra's algorithm.
//!
//! # Example
//! - [LC Shortest Path](https://judge.yosupo.jp/problem/shortest_path) (500,000 edges in 300 ms)
//!   ([Submission](https://judge.yosupo.jp/submission/165164))
//! - [DMOJ Single Source Shortest Path](https://dmoj.ca/problem/sssp)

use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

/// A graph supporting Dijkstra's algorithm. Each vertex
/// is numbered `0, 1, 2, ...`.
///
/// # Edge costs
/// The edge costs are parametrized over
/// `T: Clone + Default + Ord + Add<Output = T>` such that:
/// - `y >= T::default() --> x + y >= x`.
///
/// The edge costs must be `>= T::default()`; otherwise it
/// will panic (since Dijkstra's algorithm usually cannot
/// handle negative costs). The cost of the path is the sum
/// of the costs of the edges, in the same order as the path.
///
/// To handle `f64` costs, use `OrdF64`.
///
/// # Notation
/// When discussing the methods below, we will denote
/// `n` as the number of vertices, and
/// `m` as the number of edges.
#[derive(Clone, Debug, Default)]
pub struct Dijkstra<T> {
    pub adj: Vec<Vec<(usize, T)>>,
    pub dist: Vec<Option<T>>,
    prv: Vec<Option<usize>>,
}

impl<T> Dijkstra<T>
where
    T: Clone + Default + Ord + Add<Output = T>,
{
    /// Initializes a graph with vertices `0, ..., n-1`.
    #[inline]
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; n],
            dist: vec![],
            prv: vec![],
        }
    }

    /// Returns `n`.
    ///
    /// # Example
    /// ```
    /// # use dijkstra::Dijkstra;
    /// let graph = Dijkstra::<u32>::new(4);
    /// assert_eq!(graph.len(), 4);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.adj.len()
    }
    #[inline]
    fn verify_vertex(&self, v: usize) {
        assert!((0..self.len()).contains(&v), "invalid vertex {}", v);
    }

    /// Adds an edge `a --> b` with cost `cost`.
    ///
    /// ⚠️ If the edge is undirected, remember to also call
    /// `connect(b, a, cost)`!
    ///
    /// ⚠️ Panics if `cost < T::default()`.
    #[inline]
    pub fn connect(&mut self, a: usize, b: usize, cost: T) {
        self.verify_vertex(a);
        self.verify_vertex(b);
        assert!(cost >= T::default(), "negative cost");
        self.adj[a].push((b, cost));
    }
    /// Removes an edge `a --> b`, and returns its cost, if any.
    /// If there are multiple such edges, the first one added
    /// will be removed. Returns `None` if there are no such edges.
    ///
    /// 🕒 `O(deg(a))`.
    ///
    /// # Example
    /// ```
    /// # use dijkstra::Dijkstra;
    /// let mut graph = Dijkstra::<u32>::new(4);
    /// graph.connect(0, 1, 10);
    /// graph.connect(0, 1, 20);
    /// assert_eq!(graph.disconnect(0, 1), Some(10));
    /// assert_eq!(graph.disconnect(0, 1), Some(20));
    /// assert_eq!(graph.disconnect(0, 1), None);
    /// ```
    #[inline]
    pub fn disconnect(&mut self, a: usize, b: usize) -> Option<T> {
        let i = self.adj[a].iter().position(|&(v, _)| v == b)?;
        let (_, cost) = self.adj[a].remove(i);
        Some(cost)
    }

    /// Returns the list `dist`, where `dist[v]` is:
    /// - `Some(d)` if the minimum distance from `start` to `v` is `d`;
    /// - `None` if `v` is not reachable from `start`.
    ///
    /// The same list is stored in `self.dist`, which you can access later.
    ///
    /// 🕒 `O(n + mlogn)`.
    ///
    /// # Example
    /// ```
    /// # use dijkstra::Dijkstra;
    /// let mut graph = Dijkstra::new(4);
    /// graph.connect(0, 1, 10);
    /// graph.connect(0, 2, 20);
    /// graph.connect(1, 2, 5);
    /// assert_eq!(graph.solve(0), &vec![Some(0), Some(10), Some(15), None]);
    /// ```
    pub fn solve(&mut self, start: usize) -> &Vec<Option<T>> {
        self.verify_vertex(start);

        let mut dist = vec![None; self.len()];
        dist[start] = Some(T::default());
        self.prv = vec![None; self.len()];
        let mut pq = BinaryHeap::from([Reverse((T::default(), start))]);

        while let Some(Reverse((d, v))) = pq.pop() {
            if dist[v].as_ref() != Some(&d) {
                continue;
            }
            for (u, c) in &self.adj[v] {
                let nd = d.clone() + c.clone();
                if let Some(pd) = &dist[*u] {
                    if pd <= &nd {
                        continue;
                    }
                }
                dist[*u] = Some(nd.clone());
                self.prv[*u] = Some(v);
                pq.push(Reverse((nd, *u)));
            }
        }
        self.dist = dist;
        &self.dist
    }

    /// Returns the list representing the shortest path from
    /// `start` to `end`, where `start` is the argument for which
    /// `solve` was most recently called. If `end` is not reachable
    /// from `start`, returns `None`.
    ///
    /// 🕒 `O(n)`.
    ///
    /// ⚠️ Panics if `solve` was never called.
    ///
    /// # Example
    /// ```
    /// # use dijkstra::Dijkstra;
    /// let mut graph = Dijkstra::new(4);
    /// graph.connect(0, 1, 10);
    /// graph.connect(0, 2, 20);
    /// graph.connect(1, 2, 5);
    /// graph.solve(0);
    /// assert_eq!(graph.path(2), Some(vec![0, 1, 2]));
    /// assert_eq!(graph.path(3), None);
    /// ```
    #[must_use]
    pub fn path(&self, end: usize) -> Option<Vec<usize>> {
        assert!(!self.prv.is_empty(), "run solve(_) first");
        self.verify_vertex(end);
        self.dist[end].as_ref()?;

        let mut path = vec![end];
        while let Some(v) = path.last() {
            if let Some(u) = self.prv[*v] {
                path.push(u);
            } else {
                path.reverse();
                return Some(path);
            };
        }
        unreachable!()
    }
}
