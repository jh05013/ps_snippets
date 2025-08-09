//! Segment tree with lazy propagation.
//!
//! TODO add doc stuff

use std::fmt::Debug;

/// TODO: add custom Debug, showing only the actual values
pub struct LazySegtreeDyn<'a, V, L> {
    n: usize,
    arr: Vec<V>,
    lazy: Vec<L>,

    /// Identity value.
    id: V,
    /// Operator `x op y`.
    op: Box<dyn Fn(&V, &V) -> V + 'a>,

    /// Identity lazy value.
    not_lazy: L,
    /// Lazy operator combining `up` into `cur`.
    combine_lazy: Box<dyn Fn(&mut L, &L) + 'a>,
    /// Unlazy operator combining `lazy` into `slot`,
    /// on a node that represents the range `nl..=nr`.
    unlazy: Box<dyn Fn(&mut V, L, usize, usize) + 'a>,
}

impl<'a, V: Debug, L: Debug> Debug for LazySegtreeDyn<'a, V, L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySegtree")
            .field("n", &self.n)
            .field("arr", &self.arr)
            .field("lazy", &self.lazy)
            .finish()
    }
}

impl<'a, V: Clone, L: PartialEq + Clone> LazySegtreeDyn<'a, V, L> {
    pub fn new(
        size: usize,
        init: impl Fn(usize) -> V + 'a,
        id: V,
        op: impl Fn(&V, &V) -> V + 'a,
        not_lazy: L,
        combine_lazy: impl Fn(&mut L, &L) + 'a,
        unlazy: impl Fn(&mut V, L, usize, usize) + 'a,
    ) -> Self {
        // TODO don't use power of 2
        let mut n = 1usize;
        while n < size {
            n <<= 1;
        }
        let mut arr = vec![id.clone(); n * 2];
        let lazy = vec![not_lazy.clone(); n * 2];
        for (i, slot) in arr[n..n + size].iter_mut().enumerate() {
            *slot = init(i);
        }
        for i in (1..n).rev() {
            arr[i] = op(&arr[i * 2], &arr[i * 2 + 1]);
        }
        Self {
            n,
            arr,
            lazy,
            id,
            op: Box::new(op),
            not_lazy,
            combine_lazy: Box::new(combine_lazy),
            unlazy: Box::new(unlazy),
        }
    }

    fn propagate(&mut self, i: usize, nl: usize, nr: usize) {
        if self.lazy[i] == self.not_lazy {
            return;
        }
        let lazy = std::mem::replace(&mut self.lazy[i], self.not_lazy.clone());
        if i < self.n {
            (self.combine_lazy)(&mut self.lazy[i * 2], &lazy);
            (self.combine_lazy)(&mut self.lazy[i * 2 + 1], &lazy);
        }
        (self.unlazy)(&mut self.arr[i], lazy, nl, nr);
    }

    pub fn update(&mut self, l: usize, r: usize, lazy: L) {
        self.update_inner(l, r, &lazy, 1, 0, self.n - 1);
    }
    fn update_inner(&mut self, l: usize, r: usize, lazy: &L, i: usize, nl: usize, nr: usize) {
        self.propagate(i, nl, nr);
        if r < nl || nr < l {
            return;
        }
        if l <= nl && nr <= r {
            (self.combine_lazy)(&mut self.lazy[i], lazy);
            self.propagate(i, nl, nr);
            return;
        }
        let m = (nl + nr) / 2;
        self.update_inner(l, r, lazy, i * 2, nl, m);
        self.update_inner(l, r, lazy, i * 2 + 1, m + 1, nr);
        self.arr[i] = (self.op)(&self.arr[i * 2], &self.arr[i * 2 + 1]);
    }

    pub fn query(&mut self, l: usize, r: usize) -> V {
        self.query_inner(l, r, 1, 0, self.n - 1)
    }
    fn query_inner(&mut self, l: usize, r: usize, i: usize, nl: usize, nr: usize) -> V {
        self.propagate(i, nl, nr);
        if r < nl || nr < l {
            return self.id.clone();
        }
        if l <= nl && nr <= r {
            return self.arr[i].clone();
        }
        let m = (nl + nr) / 2;
        let left = self.query_inner(l, r, i * 2, nl, m);
        let right = self.query_inner(l, r, i * 2 + 1, m + 1, nr);
        (self.op)(&left, &right)
    }
}
