//! Explanation

use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{Debug, Display},
    ops::*,
    str::FromStr,
};

extern crate gcd;
use gcd::Gcd;

/// Fraction with 64-bit numerators and denominators.
///
/// TODO: add overflow behavior, docs, tests, etc.
///
/// # Examples
/// - Parsing
///   - see eolymp 9425, 9426, 9427, 9428 below
/// - Ordering
///   - [eolymp 1031 Ordered Fractions](https://basecamp.eolymp.com/en/problems/1031)
///   - [eolymp 9428 Fractions: minimum and maximum](https://basecamp.eolymp.com/en/problems/9428)
/// - Arithmetic (TODO)
///   - [eolymp 7363 Sum of fractions](https://basecamp.eolymp.com/en/problems/7363)
///   - [eolymp 9425 Fractions: addition and subtraction](https://basecamp.eolymp.com/en/problems/9425)
///   - [eolymp 9426 Fractions: multiplication and division](https://basecamp.eolymp.com/en/problems/9426)
///   - [eolymp 9427 Fractions: n-th term of the sequence](https://basecamp.eolymp.com/en/problems/9427)
#[derive(Copy, Clone)]
pub struct Frac64 {
    numer: i64,
    /// NOTE: internally, this should be <= i64::MAX
    denom: u64,
}

impl Display for Frac64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numer, self.denom)
    }
}

impl Debug for Frac64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Default for Frac64 {
    fn default() -> Self {
        Self { numer: 0, denom: 1 }
    }
}

impl From<i64> for Frac64 {
    fn from(n: i64) -> Self {
        Self { numer: n, denom: 1 }
    }
}

impl From<Frac64> for f64 {
    fn from(f: Frac64) -> Self {
        (f.numer as f64) / (f.denom as f64)
    }
}

impl TryFrom<Frac64> for i64 {
    type Error = (); // TODO

    fn try_from(f: Frac64) -> Result<Self, Self::Error> {
        let (num, den) = (f.numer, f.denom as i64);
        if num % den != 0 {
            return Err(());
        }
        Ok(num / den)
    }
}

impl FromStr for Frac64 {
    type Err = (); // TODO

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');

        // numerator
        let numer = split.next();
        if numer.is_none() {
            return Err(());
        }
        let numer: Result<i64, _> = numer.unwrap().parse();
        if numer.is_err() {
            return Err(());
        }
        let numer = numer.unwrap();

        // denominator
        let denom = split.next();
        if denom.is_none() {
            // treat as integer
            return Ok(Self { numer, denom: 1 });
        }
        let denom: Result<i64, _> = denom.unwrap().parse();
        if denom.is_err() {
            return Err(());
        }
        let denom = denom.unwrap();

        if split.next().is_some() {
            return Err(());
        }
        Ok(Self::new(numer, denom))
    }
}

impl PartialEq for Frac64 {
    fn eq(&self, other: &Self) -> bool {
        (self.numer as i128) * (other.denom as i128) == (other.numer as i128) * (self.denom as i128)
    }
}

impl Eq for Frac64 {}

impl PartialOrd for Frac64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Frac64 {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = (self.numer as i128) * (other.denom as i128);
        let right = (other.numer as i128) * (self.denom as i128);
        left.cmp(&right)
    }
}

impl Neg for Frac64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numer: -self.numer,
            denom: self.denom,
        }
    }
}

impl Add for Frac64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = (self.numer, self.denom);
        let (c, d) = (rhs.numer, rhs.denom);

        // optimize numbers without gcd
        if b == d {
            return Self {
                numer: a + c,
                denom: b,
            };
        }
        if b > d && b % d == 0 {
            return Self {
                numer: c * ((b / d) as i64) + a,
                denom: b,
            };
        }
        if d > b && d % b == 0 {
            return Self {
                numer: a * ((d / b) as i64) + c,
                denom: d,
            };
        }

        Self {
            numer: a * (d as i64) + c * (b as i64),
            denom: b * d,
        }
    }
}

impl AddAssign for Frac64 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Frac64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Frac64 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Frac64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            numer: self.numer * rhs.numer,
            denom: self.denom * rhs.denom,
        }
    }
}

impl MulAssign for Frac64 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Frac64 {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl DivAssign for Frac64 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Frac64 {
    pub fn new(numer: i64, denom: i64) -> Self {
        assert_ne!(denom, 0);
        if denom < 0 {
            Self {
                numer: -numer,
                denom: (-denom) as u64,
            }
        } else {
            Self {
                numer,
                denom: denom as u64,
            }
        }
    }

    pub fn numer(&self) -> i64 {
        self.numer
    }

    pub fn denom(&self) -> u64 {
        self.denom
    }

    pub fn inverse(&self) -> Self {
        Self::new(self.denom as i64, self.numer)
    }

    fn gcd(&self) -> u64 {
        self.numer.unsigned_abs().gcd(self.denom)
    }

    pub fn is_irreducible(&self) -> bool {
        self.gcd() == 1
    }

    pub fn reduce(&mut self) {
        let g = self.gcd();
        self.numer /= g as i64;
        self.denom /= g;
    }

    pub fn reduced(mut self) -> Self {
        self.reduce();
        self
    }
}
