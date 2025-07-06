//! Tarjan's strongly connected components algorithm.
//!
//! # Examples
//! - [eolymp 1667 Graph condensation](https://basecamp.eolymp.com/en/problems/1667)
//! ```ignore
//! let n = oj.usize();
//! let mut graph = UnweightedGraph::new(n);
//! for _ in 0..oj.usize() {
//!     graph.direct(oj.usize()-1, oj.usize()-1, ());
//! }
//! let scc = make_scc(&graph);
//! oj.write(scc.count).ln();
//! for x in scc.indices { oj.write(x+1).sp(); }
//! oj.ln();
//! ```
//! - [LC Strongly Connected Components](https://judge.yosupo.jp/problem/scc)
//! ```ignore
//! let n = oj.usize();
//! let mut graph = UnweightedGraph::new(n);
//! for _ in 0..oj.usize() {
//!     graph.direct(oj.usize(), oj.usize(), ());
//! }
//! let comps = make_scc(&graph).components();
//! oj.write(comps.len()).ln();
//! for comp in comps {
//!     oj.write(comp.len()).sp();
//!     for x in comp { oj.write(x).sp(); }
//!     oj.ln();
//! }
//! ```

extern crate edge_list;

use edge_list::Graph;
use std::convert::TryInto;

struct SccProcessor<'a, T> {
    graph: &'a Graph<T>,
    up: Vec<u32>,
    visit: Vec<u32>,
    scx: Vec<u32>,
    stack: Vec<u32>,
    cnt: u32,
    sccnt: u32,
}

const NOPE: u32 = u32::MAX;
impl<'a, T> SccProcessor<'a, T> {
    fn dfs(&mut self, v: u32) {
        let vs = v as usize;
        self.up[vs] = self.cnt;
        self.visit[vs] = self.cnt;
        self.cnt += 1;
        self.stack.push(v);
        for (nxt, _) in self.graph.neighbors(vs) {
            if self.visit[nxt] == NOPE {
                self.dfs(nxt.try_into().unwrap());
                self.up[vs] = self.up[vs].min(self.up[nxt]);
            } else if self.scx[nxt] == NOPE {
                self.up[vs] = self.up[vs].min(self.visit[nxt]);
            }
        }
        if self.up[vs] == self.visit[vs] {
            while let Some(t) = self.stack.pop() {
                self.scx[t as usize] = self.sccnt;
                if t == v {
                    break;
                }
            }
            self.sccnt += 1;
        }
    }
}

/// The SCC information.
#[derive(Debug, Clone)]
pub struct Scc {
    /// The number of components.
    pub count: usize,
    /// `indices[v]` is the indices of the component
    /// that contains vertex `v`.
    /// The indices are in **topological** order.
    pub indices: Vec<u32>,
}

/// Returns the SCC of `graph`.
pub fn make_scc<T>(graph: &Graph<T>) -> Scc {
    let n = graph.vertex_count();
    let mut me = SccProcessor {
        graph,
        up: vec![NOPE; n],
        visit: vec![NOPE; n],
        scx: vec![NOPE; n],
        stack: vec![NOPE; n],
        cnt: 0,
        sccnt: 0,
    };
    for v in 0..n {
        if me.visit[v] == NOPE {
            me.dfs(v.try_into().unwrap());
        }
    }
    let count = me.sccnt;
    for idx in &mut me.scx {
        *idx = count - 1 - *idx;
    }

    Scc {
        count: count as usize,
        indices: me.scx,
    }
}

impl Scc {
    /// Returns the strongly connected components
    /// in **topological** order.
    /// ```
    /// # extern crate edge_list;
    /// # use scc::make_scc;
    /// # use edge_list::UnweightedGraph;
    /// let mut graph = UnweightedGraph::new(3);
    /// graph.direct(0, 1, ());
    /// graph.direct(1, 0, ());
    /// graph.direct(0, 2, ());
    /// let comps = make_scc(&graph).components();
    /// assert!(comps[0] == vec![0, 1] || comps[0] == vec![1, 0]);
    /// assert_eq!(comps[1], vec![2]);
    /// ```
    pub fn components(&self) -> Vec<Vec<usize>> {
        let mut comps = vec![vec![]; self.count];
        for (v, idx) in self.indices.iter().enumerate() {
            comps[*idx as usize].push(v);
        }
        comps
    }
}
