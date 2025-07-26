//! [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree).
//!
//! # Usage
//! A Fenwick tree is defined over a type `V` and an operation `op`
//! that together form a commutative semigroup:
//! - **Associativity**: `a op (b op c) = (a op b) op c`
//! - **Commutativity**: `a op b = b op a`
//!
//! Create a new type (likely zero-sized), and implement [`FenwickOp`]
//! for it. See examples below.
//!
//! Furthermore, if the following **cancellativity** holds,
//! then we can also implement [`InvOp`] that cancels `val` from `cur`.
//! - `a op b = a op c ==> b = c`
//!
//! ## Common Types
//! Here we collect common operators for Fenwick trees.
//! Copy and paste at your leisure.
//! - Integers with addition: already implemented for `i64`, `u64`, etc.
//! - Integers with maximum
//! ```ignore
//! struct MaxI64;
//! impl FenwickOp for MaxI64 {
//!     type V = i64;
//!     fn add(cur: &mut Self::V, val: &Self::V) {
//!         *cur = (*cur).max(*val);
//!     }
//! }
//! ```
//!
//! # Examples
//! - [LC Point Add Range Sum](https://judge.yosupo.jp/submission/299105)
//! ```ignore
//! fn main() {
//! 	let mut oj = io_short::stdin();
//!
//! 	let n = oj.usize();
//! 	let q = oj.usize();
//! 	let mut fen = Fenwick::<i64>::from(oj.vec(n));
//! 	for _ in 0..q {
//! 		let qty = oj.usize();
//! 		if qty == 0 { fen.add(oj.usize(), oj.i64()); }
//! 		else {
//! 			let ans = fen.sum(oj.usize(), oj.usize()-1);
//! 			oj.write(ans).ln();
//! 		}
//! 	}
//! }
//! ```

pub trait FenwickOp {
    type V: Clone;
    fn add(cur: &mut Self::V, val: &Self::V);
}

pub trait InvOp: FenwickOp {
    fn sub(cur: &mut <Self as FenwickOp>::V, val: &<Self as FenwickOp>::V);
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fenwick<T: FenwickOp>(Vec<T::V>);

impl<T: FenwickOp> From<Vec<T::V>> for Fenwick<T> {
    fn from(mut value: Vec<T::V>) -> Self {
        for i in 0..value.len() {
            let v = &value[i].clone();
            if let Some(cur) = value.get_mut(i | (i + 1)) {
                T::add(cur, v);
            }
        }
        Self(value)
    }
}

impl<T: FenwickOp> Fenwick<T> {
    pub fn new_from_identity(n: usize, identity: T::V) -> Self {
        Self(vec![identity; n])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn add(&mut self, mut i: usize, v: &T::V) {
        assert!(i < self.len());
        while let Some(cur) = self.0.get_mut(i) {
            T::add(cur, v);
            i |= i + 1;
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T::V {
        assert!(i < self.len());
        let mut ans = self.0[i].clone();
        loop {
            i &= i + 1;
            i = if i != 0 { i - 1 } else { return ans };
            T::add(&mut ans, &self.0[i]);
        }
    }
}

impl<T: FenwickOp + InvOp> Fenwick<T> {
    pub fn sum(&self, l: usize, r: usize) -> T::V {
        assert!(l <= r, "Bad query range [{}, {}]", l, r);
        let mut vr = self.prefix_sum(r);
        if l != 0 {
            T::sub(&mut vr, &self.prefix_sum(l - 1));
        }
        vr
    }
}

// Usual sum impl
macro_rules! impl_fenwick_sum {
	($($T:ty) *) => { $(
		impl FenwickOp for $T {
			type V = $T;
			fn add(cur: &mut $T, val: &$T) { *cur+=*val; }
		}
		impl InvOp for $T {
			fn sub(cur: &mut $T, val: &$T) { *cur-=*val; }
		}
	)* };
}
impl_fenwick_sum!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

#[cfg(test)]
mod tests {
    use crate::{Fenwick, FenwickOp, InvOp};

    enum Query<T: FenwickOp> {
        Add(usize, T::V),
        Sum(usize, usize),
    }

    fn do_test_fenwick<T>(mut initial: Vec<T::V>, queries: Vec<Query<T>>, identity: T::V)
    where
        T: FenwickOp + InvOp,
        <T as FenwickOp>::V: PartialEq + std::fmt::Debug,
    {
        let mut fen = Fenwick::<T>::from(initial.clone());
        let mut brute = initial;
        for (qi, query) in queries.into_iter().enumerate() {
            match query {
                Query::Add(index, val) => {
                    fen.add(index, &val);
                    T::add(&mut brute[index], &val);
                }
                Query::Sum(l, r) => {
                    let val = fen.sum(l, r);
                    let mut ans = identity.clone();
                    for x in &brute[l..=r] {
                        T::add(&mut ans, x);
                    }
                    assert_eq!(val, ans, "query {qi}");
                }
            }
        }
    }

    #[test]
    fn test_fenwick_u64() {
        do_test_fenwick(
            vec![2u64, 1, 5, 3, 4, 3, 1, 7, 4, 5],
            vec![
                Query::<u64>::Sum(0, 9),
                Query::Sum(2, 7),
                Query::Add(3, 12),
                Query::Sum(2, 6),
                Query::Add(0, 60),
                Query::Sum(0, 0),
                Query::Sum(0, 4),
                Query::Add(9, 7),
                Query::Sum(5, 9),
                Query::Sum(3, 8),
            ],
            0,
        )
    }
}
