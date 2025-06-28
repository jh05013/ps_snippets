//! [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search).
//!
//! Uses [`Graph`].

extern crate edge_list;

use std::collections::VecDeque;

use edge_list::Graph;

/// Returns a list `D` of size `|V(G)|`.
/// `D[i]` is `Some(d)` if the path from `v` to `i`
/// with minimum number of edges consists of `d` edges,
/// or `None` if `i` is unreachable.
///
/// This function can be applied to any [`Graph`] and
/// not just unweighted graphs. However, the weights
/// will be ignored as if every weight is 1.
///
/// 🕒 `O(V+E)`.
///
/// TODO: support multiple starts.
/// TODO: support path recovery.
///
/// # Example
/// ```
/// # extern crate edge_list;
/// # use bfs::bfs;
/// # use edge_list::UnweightedGraph;
/// let mut graph = UnweightedGraph::new(4);
/// graph.direct(0, 1, ());
/// graph.direct(1, 3, ());
/// graph.direct(3, 0, ());
/// let dist = bfs(&graph, 0);
/// assert_eq!(dist, vec![Some(0), Some(1), None, Some(2)]);
/// ```
pub fn bfs<T>(graph: &Graph<T>, v: usize) -> Vec<Option<u32>> {
    let mut dist = vec![None; graph.vertex_count()];
    dist[v] = Some(0);
    let mut queue = VecDeque::from([v]);
    while let Some(from) = queue.pop_front() {
        let d = dist[from].unwrap();
        for (to, _) in graph.neighbors(from) {
            if let Some(nd) = dist[to] {
                if nd <= d + 1 {
                    continue;
                }
            }
            dist[to] = Some(d + 1);
            queue.push_back(to);
        }
    }
    dist
}
