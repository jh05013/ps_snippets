/// A graph DS for properties of sum(min(d(u), d(v))) over edges uv.
/// 
/// The value of that expression is $O(E \sqrt E)$, making it possible
/// to e.g. enumerate all triangles in a simple graph.
/// 
/// # Example
/// TODO
/// 
/// # Practice Problems
/// - [LC Enumerate Triangles](https://judge.yosupo.jp/problem/enumerate_triangles)
/// - [BOJ 2390 ⎐](https://www.acmicpc.net/problem/2390)
/// - [BOJ 14571 모래시계](https://www.acmicpc.net/problem/14571) in $O(E \sqrt E)$
/// - [BOJ 28200 4](https://www.acmicpc.net/problem/28200)
pub mod degree_ordered_graph;

/// Maximum flow using Dinic's algorithm.
/// 
/// # Initialization & Building
/// - `new(n)` initializes a graph with `n` vertices.
/// - `connect(s, e, cap)` connects from vertex `s` to `e` with capacity `cap`,
///   and returns an `EdgeIndex` (see below).
/// 
/// # Solving
/// - `max_flow(src, sink)` sends and returns the maximum flow from vertex `src` to `sink`.
///   After this, indexing `Dinic` by `EdgeIndex` to get an `Edge` and calling `used()`
///   gives the amount of flow sent along that edge.
/// - `min_cut(src, sink)` sends the maximum flow from vertex `src` to `sink`,
///   and returns the amount, along with the minimum cut between `src` to `sink`.
///   The minimum cut is represented by the list of vertices in the `src` side.
/// 
/// # Caution
/// The flow is not reset between successive `max_flow` or `min_cut` calls.
/// 
/// # Time Complexity
/// TODO
/// 
/// # Example
/// TODO
/// 
/// # Practice Problems
/// TODO
pub mod dinic;

/// Minimum cost maximum flow.
/// 
/// The base code was taken from [bamgoeSN](https://bamgoesn.github.io/rust-ps-md/graphs/flow/mcmf.html).
/// 
/// TODO document.
pub mod mcmf;

/// MST in a dense graph in $O(n^2)$ time using Prim's algorithm.
/// 
/// To use it, first define a *cost function*:
/// `F(i, j) = Some(v)` if there is an edge `i-j` with cost `v`,
/// otherwise `None`.
/// Then `dense_mst(n, F)` returns the `Mst` on the graph
/// whose weights are defined by `F`.
/// 
/// The `Mst` struct has the following fields:
/// - `is_connected`, representing whether the graph is connected;
/// - `total_cost`, the sum of the weights of the chosen edges;
/// - `edges`, the chosen edges, in the form of `(cost, vertex, vertex)`.
/// 
/// (Yes, this means this should technically be called `dense_msf`
/// since it actually finds the minimum spanning forest.)
/// 
/// # Caution
/// 
/// - If `F` is not symmetric (i.e. `F(i, j) != F(j, i)`),
///   the value `F(i, j)` is used where `i < j`.
/// - `mst.edges` is not necessarily sorted by cost.
/// 
/// # Example
/// ```
/// # use ps_snippets::graph::dense_mst::*;
/// // 0 1 2 3
/// // 1 0 3 2    [1] [2] [1]
/// // 2 3 0 1   0---1---3---2
/// // 3 2 1 0
/// let xor = |i, j| { Some(i^j) };
/// let mst = dense_mst(4, xor);
/// assert!(mst.is_connected);
/// assert_eq!(mst.total_cost, 1+2+1);
/// assert_eq!(mst.edges.len(), 3);
/// 
/// // . 1 . .
/// // 1 . . .    [1]     [1]
/// // . . . 1   0---1   2---3
/// // . . 1 .
/// let xor2 = |i, j| { if i^j == 1 { Some(i^j) } else { None } };
/// let mst2 = dense_mst(4, xor2);
/// assert!(!mst2.is_connected);
/// assert_eq!(mst2.total_cost, 1+1);
/// assert_eq!(mst2.edges.len(), 2);
/// ```
/// 
/// # Practice Problems
/// - [BOJ 20390 완전그래프의 최소 스패닝 트리](https://www.acmicpc.net/problem/20390)
/// - [USACO Mar2014 Silver1 Watering the Fields](https://www.acmicpc.net/problem/10021)
/// - [USACO Open2019 Gold2 I Would Walk 500 Miles](https://www.acmicpc.net/problem/17193)
pub mod dense_mst;
