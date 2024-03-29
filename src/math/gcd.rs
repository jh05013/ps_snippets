pub mod gcd_mod {
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
				if self == 0 || b == 0 { return self | b; }
				let (mut a, mut b) = (self, b);
				let gcd_exponent_on_two = (a | b).trailing_zeros();
				a >>= a.trailing_zeros(); b >>= b.trailing_zeros();
				while a != b {
					if a < b { core::mem::swap(&mut a, &mut b); }
					a -= b; a >>= a.trailing_zeros();
				}
				a << gcd_exponent_on_two
			} }
			impl Lcm for $T { fn lcm(self, b: Self) -> Self {
				if b == 0 { 0 } else { self / self.gcd(b) * b }
			} }
		)* };
	}
	impl_gcd_lcm!(u8 u16 u32 u64 u128 usize);
}
pub use gcd_mod::{Gcd, Lcm};
