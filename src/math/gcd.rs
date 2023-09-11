//! Greatest common divisor and least common multiple.
//! - `a.gcd(b)` is the greatest common divisor of `a` and `b`.
//!   If either a or b is 0, assume their GCD is the other one.
//! - `a.lcm(b)` is the least common multiple of `a` and `b`.
//!   If either a or b is 0, assume their LCM is 0.
//! 
//! 🕒 $O(\log a + \log b)$ for primitive unsigned integers.
//! 
//! # Examples
//! ```
//! use ps_snippets::math::gcd::*;
//! 
//! assert_eq!(6u32.gcd(4), 2);
//! assert_eq!(6u64.lcm(4), 12);
//! assert_eq!(0u32.gcd(4), 4);
//! assert_eq!(4u32.gcd(0), 4);
//! ```
//! 
//! # Practice Problems
//! - [eolymp 571 The greatest common divisor](https://www.eolymp.com/en/problems/571) `gcd`
//! - [eolymp 135 LCM](https://www.eolymp.com/en/problems/135) `lcm` (use `u128`)
//! - [PE 5 Smallest Multiple](https://projecteuler.net/problem=5) `lcm`

/// Greatest common divisor.
pub trait Gcd {
	/// `a.gcd(b)` is the greatest common divisor of `a` and `b`.
	/// If either a or b is 0, assume their GCD is the other one.
	fn gcd(self, other: Self) -> Self;
}

/// Least common multiple.
pub trait Lcm {
	/// `a.lcm(b)` is the least common multiple of `a` and `b`.
	/// If either a or b is 0, assume their LCM is 0.
	fn lcm(self, other: Self) -> Self;
}

macro_rules! impl_gcd_lcm {
	($($T:ty) *) => { $(
		impl Gcd for $T { fn gcd(self, b: Self) -> Self {
			if b == 0 { self } else { b.gcd(self % b) }
		} }
		impl Lcm for $T { fn lcm(self, b: Self) -> Self {
			if b == 0 { 0 } else { self * b / self.gcd(b) }
		} }
	)* };
}
impl_gcd_lcm!(u8 u16 u32 u64 u128 usize);
