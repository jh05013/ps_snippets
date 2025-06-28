//! [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search).
//!
//! Uses [`Graph`].

extern crate edge_list;

use edge_list::Graph;

/// Given an undirected graph with costs
/// (1) `false`: two vertices must have the same color, and
/// (2) `true`: two vertices must have the different color,
/// determines whether the graph is bicolorable.
/// If so, returns the lexicographically first coloring.
/// Otherwise, returns `None`.
/// 
/// 🕒 `O(V+E)`.
/// 
/// TODO: generalize by `T`, passing a predicate.
/// 
/// TODO: add examples.
pub fn bicolor_graph(graph: &Graph<bool>) -> Option<Vec<bool>> {
    let mut color = vec![None; graph.vertex_count()];
    for start in 0..graph.vertex_count() {
        if color[start].is_some() {
            continue;
        }
        let mut stack = vec![start];
        color[start] = Some(false);
        while let Some(a) = stack.pop() {
            let col_a = color[a].unwrap();
            for (b, cost) in graph.neighbors(a) {
                let col_b = col_a != *cost;
                if color[b] == Some(!col_b) {
                    return None;
                }
                if color[b].is_none() {
                    stack.push(b);
                }
                color[b] = Some(col_b);
            }
        }
    }
    Some(color.into_iter().map(|b| b.unwrap()).collect())
}
