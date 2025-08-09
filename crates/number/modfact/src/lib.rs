//! Modulo factorial, an extension of [`modint`].

use std::ops::Index;

use modint::Modint;

extern crate modint;

/// Modulo factorial interface.
///
/// ⚠️ May panic if `MOD` is not prime. TODO: make it better
#[derive(Clone, Debug)]
pub struct Modfact<const MOD: u32> {
    /// `fact[i] = i! mod MOD`.
    pub fact: Vec<Modint<MOD>>,
    /// Modulo inverse of `fact[i]`.
    pub ifact: Vec<Modint<MOD>>,
}

pub type Modfact17 = Modfact<1000000007>;
pub type Modfact99 = Modfact<998244353>;

impl<const MOD: u32> Index<usize> for Modfact<MOD> {
    type Output = Modint<MOD>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.fact[index]
    }
}

impl<const MOD: u32> Modfact<MOD> {
    /// Creates an instance that supports factorials from 0 to `n`.
    ///
    /// 🕒 `O(n + log MOD)`.
    ///
    /// ⚠️ Panics if `n >= MOD`. TODO: make it better
    pub fn new(n: usize) -> Self {
        let mut fact = vec![Modint::<MOD>::from(1u32); n + 1];
        let mut ifact = vec![Modint::<MOD>::from(1u32); n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * (i.into());
        }
        ifact[n] = fact[n].inv();
        for i in (2..n).rev() {
            ifact[i] = ifact[i + 1] * ((i + 1).into());
        }
        Self { fact, ifact }
    }

    /// Returns `n choose r`.
    /// If `n < r`, the answer is 0.
    ///
    /// ⚠️ Unspeicifed behavior if `MOD` is not prime.
    pub fn binom(&self, n: usize, r: usize) -> Modint<MOD> {
        if n < r {
            return Modint::default();
        };
        self[n] * self.ifact[r] * self.ifact[n - r]
    }

    /// Returns the modulo inverse of `i` (in constant time!)
    ///
    /// ⚠️ `i` must be within the range of this instance.
    /// Also panics if `i == 0`.
    ///
    /// ⚠️ Unspeicifed behavior if `MOD` is not prime.
    pub fn inv(&self, i: usize) -> Modint<MOD> {
        assert!(i > 0);
        self.fact[i - 1] * self.ifact[i]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modfact() {
        let modfact = Modfact::<101>::new(100);
        assert_eq!(1u32, modfact[0].into());
        assert_eq!(1u32, modfact[1].into());
        assert_eq!(24u32, modfact[4].into());
        assert_eq!(19u32, modfact[5].into());
        assert_eq!(100u32, modfact[100].into());
    }

    #[test]
    fn test_modfact_inv() {
        let modfact = Modfact::<1000000007>::new(10000);
        for i in 0..10000 {
            assert_eq!(modfact[i] * modfact.ifact[i], 1u32.into());
        }
        for i in 1..10000 {
            assert_eq!(Modint::from(i) * modfact.inv(i), 1u32.into());
        }
    }
}
