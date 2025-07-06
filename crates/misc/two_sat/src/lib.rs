//! 2-SAT.
//! # Examples
//! - [LC 2 Sat](https://judge.yosupo.jp/problem/two_sat)
//! - [BOJ 11281 2-SAT - 4](https://www.acmicpc.net/problem/11281)
//! ```ignore
//! let mut cnf = TwoSat::new(oj.usize());
//! for _ in 0..oj.usize() {
//!     let x = oj.i32();
//!     let (x, sign_x) = (x.abs() as usize - 1, x > 0);
//!     let y = oj.i32();
//!     let (y, sign_y) = (y.abs() as usize - 1, y > 0);
//!     cnf.clause(x, sign_x, y, sign_y);
//! }
//! let Some(sol) = cnf.solution() else {
//!     oj.write(0).ln();
//!     return;
//! };
//! oj.write(1).ln();
//! for x in sol {
//!     oj.write(x as i32).sp();
//! }
//! ```

extern crate edge_list;
extern crate scc;

use std::fmt::Debug;

use edge_list::UnweightedGraph;
use scc::make_scc;

/// 2-SAT interface.
#[derive(Clone, Debug)]
pub struct TwoSat {
    graph: UnweightedGraph,
}

impl TwoSat {
    /// Initializes a 2-SAT interface with `var_count` variables.
    #[inline]
    pub fn new(var_count: usize) -> Self {
        Self {
            graph: UnweightedGraph::new(var_count * 2),
        }
    }

    /// Returns the number of variables.
    /// ```
    /// # use two_sat::TwoSat;
    /// let cnf = TwoSat::new(10);
    /// assert_eq!(cnf.num_vars(), 10);
    /// ```
    #[inline]
    pub fn num_vars(&self) -> usize {
        self.graph.vertex_count() >> 1
    }

    #[inline]
    fn var(v: usize, sign: bool) -> usize {
        v * 2 + (sign as usize)
    }

    /// Adds a clause `X \/ Y`,
    /// where `X` is `x` if `sign_x` is true,
    /// otherwise `~x`; and similarly for `Y`.
    #[inline]
    pub fn clause(&mut self, x: usize, sign_x: bool, y: usize, sign_y: bool) {
        self.graph
            .direct(Self::var(x, !sign_x), Self::var(y, sign_y), ());
        self.graph
            .direct(Self::var(y, !sign_y), Self::var(x, sign_x), ());
    }

    /// Adds `X => Y`, i.e. `~X \/ Y`,
    /// where `X` is `x` if `sign_x` is true,
    /// otherwise `~x`; and similarly for `Y`.
    #[inline]
    pub fn if_then(&mut self, x: usize, sign_x: bool, y: usize, sign_y: bool) {
        self.clause(x, !sign_x, y, sign_y)
    }

    /// Adds `~(X /\ Y)`, i.e. `~X \/ ~Y`,
    /// where `X` is `x` if `sign_x` is true,
    /// otherwise `~x`; and similarly for `Y`.
    #[inline]
    pub fn not_both(&mut self, x: usize, sign_x: bool, y: usize, sign_y: bool) {
        self.clause(x, !sign_x, y, !sign_y)
    }

    /// Returns whether the 2-CNF is satisfiable.
    /// ```
    /// # use two_sat::TwoSat;
    /// let mut cnf = TwoSat::new(2);
    /// cnf.clause(0, true, 1, true);
    /// cnf.clause(0, true, 1, false);
    /// assert!(cnf.is_satisfiable());
    /// cnf.clause(0, false, 1, true);
    /// cnf.clause(0, false, 1, false);
    /// assert!(!cnf.is_satisfiable());
    /// ```
    pub fn is_satisfiable(&self) -> bool {
        make_scc(&self.graph)
            .indices
            .chunks(2)
            .all(|c| c[0] != c[1])
    }

    /// If the 2-CNF is satisfiable, returns one of the solutions.
    /// Otherwise, returns `None`.
    /// ```
    /// # use two_sat::TwoSat;
    /// let mut cnf = TwoSat::new(2);
    /// cnf.clause(0, true, 1, true);
    /// cnf.clause(0, true, 1, false);
    /// let sol = cnf.solution().unwrap(); // [true, false] or [true, true]
    /// assert!(sol[0]);
    /// cnf.clause(0, false, 1, true);
    /// cnf.clause(0, false, 1, false);
    /// assert!(cnf.solution().is_none());
    /// ```
    pub fn solution(&self) -> Option<Vec<bool>> {
        let scc = make_scc(&self.graph);
        if scc.indices.chunks(2).any(|c| c[0] == c[1]) {
            return None;
        }

        let mut sol = vec![None; self.num_vars()];
        // in reverse topological order,
        // if someone's not set, set it to the literal's sign
        for component in scc.components() {
            for lit in component {
                let id = lit >> 1;
                if sol[id].is_some() {
                    continue;
                }
                sol[id] = Some(lit & 1 == 0);
            }
        }
        Some(sol.into_iter().map(|b| b.unwrap()).collect())
    }
}
