//! Greatest common divisor (GCD) and
//! Least common multiple (LCM).
//!
//! Both traits are implemented for primitve unsigned integers
//! (`u8` through `u128` and `usize`) using the
//! [binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).
//! It still takes logarithmic time, but should be faster than
//! Euclidean algorithm in practice.
//!
//! # Examples
//! - [KOI 최대공약수와 최소공배수](https://www.acmicpc.net/problem/2609)
//! ```ignore
//! use gcd::{Gcd, Lcm};
//! let a = oj.u32();
//! let b = oj.u32();
//! oj.write(a.gcd(b)).ln().write(a.lcm(b));
//! ```
//!
//! - [PE 5 Smallest Multiple](https://projecteuler.net/problem=5)
//! ```
//! use gcd::Lcm;
//! let ans = (1u64..=20u64).fold(1u64, |x, acc| x.lcm(acc));
//! assert_eq!(ans, 232792560);
//! ```

/// Greatest common divisor.
pub trait Gcd {
    /// `a.gcd(b)` is the greatest common divisor of `a` and `b`.
    /// If either a or b is 0, assume their GCD is the other one.
    ///
    /// 🕒 `O(log(a+b))` for primitive unsigned integers.
    ///
    /// # Example
    /// ```
    /// # use gcd::Gcd;
    /// assert_eq!(12u32.gcd(18), 6);
    /// assert_eq!(12u32.gcd(0), 12);
    /// ```
    #[must_use]
    fn gcd(self, other: Self) -> Self;
}

/// Least common multiple.
pub trait Lcm {
    /// `a.lcm(b)` is the least common multiple of `a` and `b`.
    /// If either a or b is 0, assume their LCM is 0.
    ///
    /// 🕒 `O(log(a+b))` for primitive unsigned integers.
    ///
    /// # Example
    /// ```
    /// # use gcd::Lcm;
    /// assert_eq!(12u32.lcm(18), 36);
    /// assert_eq!(12u32.lcm(0), 0);
    /// ```
    #[must_use]
    fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_gcd_lcm {
	($($T:ty) *) => { $(
		impl Gcd for $T { fn gcd(self, b: Self) -> Self {
			if self == 0 || b == 0 {
				return self | b;
			}
			let (mut a, mut b) = (self, b);
			let gcd_exponent_on_two = (a | b).trailing_zeros();
			a >>= a.trailing_zeros();
			b >>= b.trailing_zeros();

			while a != b {
				if a < b {
					core::mem::swap(&mut a, &mut b);
				}
				a -= b;
				a >>= a.trailing_zeros();
			}
			a << gcd_exponent_on_two
		} }

		impl Lcm for $T { fn lcm(self, b: Self) -> Self {
			if b == 0 { 0 } else { self / self.gcd(b) * b }
		} }
	)* };
}
impl_gcd_lcm!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod test {
    use super::{Gcd, Lcm};

    fn gcd_naive(a: u32, b: u32) -> u32 {
        for v in (1..=std::cmp::max(a, b)).rev() {
            if a % v == 0 && b % v == 0 {
                return v;
            }
        }
        unreachable!();
    }

    fn gcd_naive8(a: u8, b: u8) -> u8 {
        for v in (1..=std::cmp::max(a, b)).rev() {
            if a % v == 0 && b % v == 0 {
                return v;
            }
        }
        unreachable!();
    }

    #[test]
    fn gcd_lcm_test() {
        fn lcm_naive(a: u32, b: u32) -> u32 {
            for v in 1..=b {
                if v * a % b == 0 {
                    return v * a;
                }
            }
            unreachable!();
        }

        for a in 1..100 {
            for b in 1..100 {
                assert_eq!(a.gcd(b), gcd_naive(a, b));
                assert_eq!(a.lcm(b), lcm_naive(a, b));
            }
        }
        assert_eq!((123456789012345678u64).gcd(105255401205018735), 193425723);
        assert_eq!((123456789012345678u64).lcm(10), 617283945061728390);
        assert_eq!((987654321u64).lcm(123456789), 13548070123626141);
    }

    #[test]
    fn gcd_lcm_bound_test() {
        for a in 1u8..=255 {
            for b in 1..=255 {
                assert_eq!(a.gcd(b), gcd_naive8(a, b));
                let lcm = (a as u32).lcm(b as u32);
                if lcm <= 255 {
                    assert_eq!(a.lcm(b), lcm as u8);
                }
            }
        }
    }

    #[test]
    fn gcd_lcm_zero_test() {
        for a in 0u32..100 {
            assert_eq!(a.gcd(0), a);
            assert_eq!(0.gcd(a), a);
            assert_eq!(a.lcm(0), 0);
            assert_eq!(0.lcm(a), 0);
        }
    }
}
