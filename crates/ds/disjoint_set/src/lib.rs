//! Disjoint set data structure.

pub struct DisjointSet {
    par: Vec<usize>,
}

impl DisjointSet {
    /// Initializes a new disjoint set of `{0}, {1}, ..., {n-1}`.
    pub fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
        }
    }

    /// Returns `n`.
    pub fn len(&self) -> usize {
        self.par.len()
    }

    /// Merges a set containing `a` and a set containing `b`.
    pub fn join(&mut self, a: usize, b: usize) {
        let ar = self.find(a);
        self.par[ar] = self.find(b);
    }

    /// Returns a representative element of a set containing `i`.
    ///
    /// It is guaranteed that, between any two consecutive calls of `join`,
    /// `find(a) == find(b)` iff `a` and `b` belong to the same set.
    pub fn find(&mut self, mut i: usize) -> usize {
        while self.par[i] != i {
            self.par[i] = self.par[self.par[i]];
            i = self.par[i];
        }
        i
    }
}
