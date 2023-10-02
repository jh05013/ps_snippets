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
