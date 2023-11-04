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
