//! Segment tree with lazy propagation.
//! 
//! TODO add doc stuff

use std::fmt::Debug;

pub trait LazyMonoid {
    /// Type of the value.
    type V: Clone + Debug;
    /// Type of the lazy value.
    type L: Clone + Debug + PartialEq;

    /// Identity value.
    const ID: Self::V;
    /// Operator `x op y`.
    fn op(x: &Self::V, y: &Self::V) -> Self::V;

    /// Identity lazy value.
    const NOT_LAZY: Self::L;
    /// Lazy operator combining `up` into `cur`.
    fn combine_lazy(cur: &mut Self::L, up: &Self::L);
    /// Unlazy operator combining `lazy` into `slot`,
    /// on a node that represents the range `nl..=nr`.
    fn unlazy(slot: &mut Self::V, lazy: Self::L, nl: usize, nr: usize);
}

/// TODO: add custom Debug, showing only the actual values
#[derive(Clone)]
pub struct LazySegtree<T: LazyMonoid> {
    n: usize,
    arr: Vec<T::V>,
    lazy: Vec<T::L>,
}

impl<T: LazyMonoid> Debug for LazySegtree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySegtree")
            .field("n", &self.n)
            .field("arr", &self.arr)
            .field("lazy", &self.lazy)
            .finish()
    }
}

impl<T: LazyMonoid> LazySegtree<T> {
    pub fn new(size: usize, init: impl Fn(usize) -> T::V) -> Self {
        // TODO don't use power of 2
        let mut n = 1usize;
        while n < size {
            n <<= 1;
        }
        let mut arr = vec![T::ID; n * 2];
        let lazy = vec![T::NOT_LAZY; n * 2];
        for (i, slot) in arr[n..n + size].iter_mut().enumerate() {
            *slot = init(i);
        }
        for i in (1..n).rev() {
            arr[i] = T::op(&arr[i * 2], &arr[i * 2 + 1]);
        }
        Self { n, arr, lazy }
    }

    fn propagate(&mut self, i: usize, nl: usize, nr: usize) {
        if self.lazy[i] == T::NOT_LAZY {
            return;
        }
        let lazy = std::mem::replace(&mut self.lazy[i], T::NOT_LAZY);
        if i < self.n {
            T::combine_lazy(&mut self.lazy[i * 2], &lazy);
            T::combine_lazy(&mut self.lazy[i * 2 + 1], &lazy);
        }
        T::unlazy(&mut self.arr[i], lazy, nl, nr);
    }

    pub fn update(&mut self, l: usize, r: usize, lazy: T::L) {
        self.update_inner(l, r, &lazy, 1, 0, self.n - 1);
    }
    fn update_inner(&mut self, l: usize, r: usize, lazy: &T::L, i: usize, nl: usize, nr: usize) {
        self.propagate(i, nl, nr);
        if r < nl || nr < l {
            return;
        }
        if l <= nl && nr <= r {
            T::combine_lazy(&mut self.lazy[i], lazy);
            self.propagate(i, nl, nr);
            return;
        }
        let m = (nl + nr) / 2;
        self.update_inner(l, r, lazy, i * 2, nl, m);
        self.update_inner(l, r, lazy, i * 2 + 1, m + 1, nr);
        self.arr[i] = T::op(&self.arr[i * 2], &self.arr[i * 2 + 1]);
    }

    pub fn query(&mut self, l: usize, r: usize) -> T::V {
        self.query_inner(l, r, 1, 0, self.n - 1)
    }
    fn query_inner(&mut self, l: usize, r: usize, i: usize, nl: usize, nr: usize) -> T::V {
        self.propagate(i, nl, nr);
        if r < nl || nr < l {
            return T::ID;
        }
        if l <= nl && nr <= r {
            return self.arr[i].clone();
        }
        let m = (nl + nr) / 2;
        T::op(
            &self.query_inner(l, r, i * 2, nl, m),
            &self.query_inner(l, r, i * 2 + 1, m + 1, nr),
        )
    }
}
