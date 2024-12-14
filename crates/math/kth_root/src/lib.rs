//! k-th root (i.e. `n ^ (1/k)`).
//! 
//! # Examples
//! - [LC Kth Root (Integer)](https://judge.yosupo.jp/problem/kth_root_integer)
//! ```ignore
//! for _ in 0..oj.usize() {
//!     let n = oj.u64();
//!     let k = oj.u32();
//!     oj.write(n.floor_kth_root(k)).ln();
//! }
//! ```

pub trait KthRoot: Sized {
    /// Let `x` = `self ^ (1/k)`.
    /// If `x` is an integer, returns `Ok(x)`.
    /// Otherwise, returns `Err(floor(x))`.
    ///
    /// ⚠️ Panics if `k == 0`.
    /// 
    /// # Example
    /// ```
    /// # use kth_root::KthRoot;
    /// assert_eq!(26u32.kth_root(3), Err(2));
    /// assert_eq!(27u32.kth_root(3), Ok(3));
    /// assert_eq!(28u32.kth_root(3), Err(3));
    /// ```
    fn kth_root(&self, k: u32) -> Result<Self, Self>;

    /// Returns the ceiling of `self ^ (1/k)`.
    ///
    /// ⚠️ Panics if `k == 0`.
    /// 
    /// # Example
    /// ```
    /// # use kth_root::KthRoot;
    /// assert_eq!(26u32.ceil_kth_root(3), 3);
    /// assert_eq!(27u32.ceil_kth_root(3), 3);
    /// assert_eq!(28u32.ceil_kth_root(3), 4);
    /// ```
    #[must_use]
    fn ceil_kth_root(&self, k: u32) -> Self;

    /// Returns the floor of `self ^ (1/k)`.
    ///
    /// ⚠️ Panics if `k == 0`.
    /// 
    /// # Example
    /// ```
    /// # use kth_root::KthRoot;
    /// assert_eq!(26u32.floor_kth_root(3), 2);
    /// assert_eq!(27u32.floor_kth_root(3), 3);
    /// assert_eq!(28u32.floor_kth_root(3), 3);
    /// ```
    #[must_use]
    fn floor_kth_root(&self, k: u32) -> Self {
        match self.kth_root(k) {
            Ok(v) | Err(v) => v,
        }
    }
}

macro_rules! impl_kth_root {
    ($($T:ty) *) => { $( impl KthRoot for $T {
        fn kth_root(&self, k: u32) -> Result<Self, Self> {
            use std::cmp::Ordering;
            assert_ne!(k, 0);
            if *self <= 1 || k == 1 {
                return Ok(*self);
            }

            let mut f = ((*self as f64).powf(1. / k as f64) as Self)
                .saturating_sub(1);
            while let Some(v) = (f+1).checked_pow(k) {
                match v.cmp(self) {
                    Ordering::Less => { f += 1; }
                    Ordering::Equal => { return Ok(f+1); }
                    Ordering::Greater => { return Err(f); }
                }
            }
            Err(f)
        }

        fn ceil_kth_root(&self, k: u32) -> Self {
            match self.kth_root(k) {
                Ok(v) => v,
                Err(v) => v+1,
            }
        }
    } )* }
}
impl_kth_root!(u8 u16 u32 u64 u128);

#[cfg(test)]
mod test {
    use KthRoot;

    #[test]
    fn test_kth_root() {
        macro_rules! check {
            ($n:expr, $k:expr) => {
                let floor = $n.floor_kth_root($k);
                let ceil = $n.ceil_kth_root($k);
                let ans = $n.kth_root($k);

                if $k == 1 {
                    assert_eq!(floor, $n);
                    assert_eq!(ceil, $n);
                    assert_eq!(ans, Ok($n));
                } else {
                    assert!(floor.pow($k) <= $n);
                    if let Some(v) = (floor + 1).checked_pow($k) {
                        assert!(v > $n);
                    }

                    if let Some(v) = ceil.checked_pow($k) {
                        assert!(v >= $n);
                    }
                    if let Some(v) = ceil.checked_sub(1) {
                        assert!(v.checked_pow($k).unwrap() < $n);
                    }

                    if floor.pow($k) == $n {
                        assert_eq!(ans, Ok(floor));
                    } else {
                        assert_eq!(ans, Err(floor));
                    }
                }
            };
        }

        for k in 1..100 {
            for n in 0..1000 {
                check!(n as u8, k);
                check!(n as u16, k);
                check!(n as u32, k);
                check!(n as u64, k);
                check!(n as u128, k);
                check!(u8::MAX - n as u8, k);
                check!(u16::MAX - n as u16, k);
                check!(u32::MAX - n as u32, k);
                check!(u64::MAX - n as u64, k);
                check!(u128::MAX - n as u128, k);
            }
        }
    }
}
