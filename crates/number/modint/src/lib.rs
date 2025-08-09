//! Modulo integers.

use std::{fmt::*, ops::*};

/// Unsigned integer modulo `MOD`.
///
/// ⚠️ `MOD` must be `> 0` and `<= 2147483647`.
///
/// ⚠️ `MOD` can be composite, but then `inv` and division won't work correctly!
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Modint<const MOD: u32>(u32);

pub type Modint17 = Modint<1000000007>;
pub type Modint99 = Modint<998244353>;

impl<const MOD: u32> Modint<MOD> {
    /// Creates a new modint.
    pub const fn new(n: u32) -> Self {
        Self(n % MOD)
    }

    /// Returns the integer converted to `u32`.
    #[must_use]
    pub const fn inner(&self) -> u32 {
        self.0
    }

    /// Returns `self` to the `n`-th power, modulo `MOD`.
    #[must_use]
    pub fn pow(&self, mut n: u64) -> Self {
        let mut ans = Self::from(1u32);
        let mut a = *self;
        while n != 0 {
            if n & 1 == 1 {
                ans *= a;
            }
            n >>= 1;
            a *= a;
        }
        ans
    }

    /// Returns the multiplicative inverse of `self`.
    ///
    /// If you need multiple inverses of small `n`,
    /// check out the `modfact` crate.
    ///
    /// ⚠️ Panics if `self` is 0.
    ///
    /// ⚠️ Unspecified behavior if `MOD` is not prime.
    #[must_use]
    pub fn inv(&self) -> Self {
        assert!(self.0 != 0, "Cannot invert 0");
        self.pow((MOD as u64) - 2)
    }

    /// Returns `n!` modulo `MOD`.
    ///
    /// If you need multiple factorial values of small `n`,
    /// check out the `modfact` crate.
    pub fn factorial(n: u32) -> Self {
        let mut ans = Self::from(1u32);
        for i in 2..=n {
            ans *= i.into();
        }
        ans
    }
}

// io
impl<const MOD: u32> std::str::FromStr for Modint<MOD> {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self::from(s.parse::<i64>()?))
    }
}
impl<const MOD: u32> Display for Modint<MOD> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}
impl<const MOD: u32> From<Modint<MOD>> for u32 {
    fn from(num: Modint<MOD>) -> Self {
        num.0
    }
}
impl<const MOD: u32> From<u32> for Modint<MOD> {
    fn from(num: u32) -> Self {
        Self(num.rem_euclid(MOD))
    }
}
impl<const MOD: u32> From<i32> for Modint<MOD> {
    fn from(num: i32) -> Self {
        Self(num.rem_euclid(MOD as i32) as u32)
    }
}
impl<const MOD: u32> From<i64> for Modint<MOD> {
    fn from(num: i64) -> Self {
        Self(num.rem_euclid(MOD as i64) as u32)
    }
}
impl<const MOD: u32> From<u64> for Modint<MOD> {
    fn from(num: u64) -> Self {
        Self(num.rem_euclid(MOD as u64) as u32)
    }
}
impl<const MOD: u32> From<usize> for Modint<MOD> {
    fn from(num: usize) -> Self {
        Self(num.rem_euclid(MOD as usize) as u32)
    }
}

// arithmetic
impl<const MOD: u32> Neg for Modint<MOD> {
    type Output = Self;
    fn neg(self) -> Self {
        Self(MOD - self.0)
    }
}
impl<const MOD: u32> AddAssign for Modint<MOD> {
    fn add_assign(&mut self, b: Self) {
        self.0 += b.0;
        if self.0 >= MOD {
            self.0 -= MOD;
        }
    }
}
impl<const MOD: u32> Add for Modint<MOD> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        let mut z = self;
        z += b;
        z
    }
}
impl<const MOD: u32> SubAssign for Modint<MOD> {
    fn sub_assign(&mut self, b: Self) {
        *self += -b;
    }
}
impl<const MOD: u32> Sub for Modint<MOD> {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        let mut z = self;
        z -= b;
        z
    }
}
impl<const MOD: u32> MulAssign for Modint<MOD> {
    fn mul_assign(&mut self, b: Self) {
        *self = *self * b;
    }
}
impl<const MOD: u32> Mul for Modint<MOD> {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        let val = u64::from(self.0) * u64::from(b.0) % u64::from(MOD);
        Self::from(val as u32)
    }
}
impl<const MOD: u32> DivAssign for Modint<MOD> {
    fn div_assign(&mut self, b: Self) {
        *self = *self / b;
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl<const MOD: u32> Div for Modint<MOD> {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        self * b.inv()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modint_from() {
        type M = Modint<1000000007>;
        assert_eq!(0u32, M::from(0u32).into());
        assert_eq!(0u32, M::from(1000000007u32).into());
        assert_eq!(86u32, M::from(2000000100u32).into());
    }

    #[test]
    fn test_modint_factorial() {
        type M = Modint<100>;
        assert_eq!(1u32, M::factorial(0).into());
        assert_eq!(1u32, M::factorial(1).into());
        assert_eq!(24u32, M::factorial(4).into());
        assert_eq!(20u32, M::factorial(5).into());
        assert_eq!(0u32, M::factorial(9999999).into());
    }
}
