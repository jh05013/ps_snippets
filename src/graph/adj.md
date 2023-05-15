# Adjacency List

TODO add description.

## Code
```rust,noplayground
mod adj_graph {
    #[derive(Debug, Clone)]
    pub struct AdjGraph { n: usize, bidir: bool, pub adj: Vec<Vec<usize>> }
    impl AdjGraph {
        pub fn new(n: usize, bidirectional: bool) -> Self {
            AdjGraph { n: n, bidir: bidirectional, adj: vec![vec![]; n] }
        }
        pub fn connect(&mut self, a: usize, b: usize) {
            self.adj[a].push(b);
            if self.bidir { self.adj[b].push(a); }
        }
        pub fn input_edges(&mut self, oj: &mut super::OJ, m: usize, base: usize) {
            for _ in 0..m {
                let a = oj.read::<usize>() - base;
                let b = oj.read::<usize>() - base;
                self.connect(a, b);
            }
        }
    }
    impl super::Graph for AdjGraph {
        type Neighbor<'a> = std::slice::Iter<'a, usize>;
        fn size(&self) -> usize { self.n }
        fn neighbor<'a>(&'a self, v: usize) -> Self::Neighbor<'a> { self.adj[v].iter() }
    }
} use adj_graph::*;
```