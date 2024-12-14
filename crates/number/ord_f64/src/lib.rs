//! [`f64`] but with [`Ord`] trait.
//!
//! Of course, it assumes the absence of NaN values,
//! which you wouldn't really encounter in PS.
//!
//! Supports arithmetic, printing, and derefs into `f64`.
//!
//! # Examples
//! ```
//! use ord_f64::OrdF64;
//!
//! let a = OrdF64(2.5);
//! let b = OrdF64(3.5);
//! assert_eq!(a + b, OrdF64(6.0));
//! assert_eq!(a.min(b), a); // Ord trait method
//! assert_eq!(a.sin(), 2.5_f64.sin()); // Deref f64 method
//! ```

use std::{cmp::Ordering, fmt, ops::*};

/// [`f64`] but with [`Ord`] trait.
///
/// Of course, it assumes the absence of NaN values,
/// which you wouldn't really encounter in PS.
///
/// Supports arithmetic, printing, and derefs into `f64`.
///
/// # Examples
/// ```
/// use ord_f64::OrdF64;
///
/// let a = OrdF64(2.5);
/// let b = OrdF64(3.5);
/// assert_eq!(a + b, OrdF64(6.0));
/// assert_eq!(a.min(b), a); // Ord trait method
/// assert_eq!(a.sin(), 2.5_f64.sin()); // Deref f64 method
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrdF64(pub f64);

impl fmt::Display for OrdF64 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for OrdF64 {
    type Target = f64;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for OrdF64 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Eq for OrdF64 {}
impl PartialOrd for OrdF64 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrdF64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Add for OrdF64 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for OrdF64 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for OrdF64 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for OrdF64 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Mul for OrdF64 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign for OrdF64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Div for OrdF64 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign for OrdF64 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}
impl Neg for OrdF64 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
