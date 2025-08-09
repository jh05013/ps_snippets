//! Topological sorting.
//!
//! Uses [`Graph`].

extern crate edge_list;

use edge_list::Graph;

pub fn toposort<T>(graph: &Graph<T>) -> Option<Vec<usize>> {
    let n = graph.vertex_count();
    let mut indegs = vec![0; n];
    for a in 0..n {
        for (b, _) in graph.neighbors(a) {
            indegs[b] += 1;
        }
    }

    let mut stack = (0..n).filter(|i| indegs[*i] == 0).collect::<Vec<_>>();
    let mut order = stack.clone();
    while let Some(a) = stack.pop() {
        for (b, _) in graph.neighbors(a) {
            indegs[b] -= 1;
            if indegs[b] > 0 {
                continue;
            }
            stack.push(b);
            order.push(b);
        }
    }

    (order.len() == n).then_some(order)
}
