pub mod ord_f64 {
	use std::{ops::*, fmt};

	#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
	pub struct OrdF64(pub f64);

	impl fmt::Display for OrdF64 {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{}", self.0)
		}
	}
	impl Deref for OrdF64 { type Target = f64;
		fn deref(&self) -> &Self::Target { &self.0 }
	}
	impl DerefMut for OrdF64 {
		fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
	}
	impl Eq for OrdF64 {}
	impl Ord for OrdF64 {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			self.partial_cmp(other).unwrap()
		}
	}

	impl Add for OrdF64 { type Output = OrdF64;
		fn add(self, rhs: Self) -> Self::Output { OrdF64(self.0 + rhs.0) }
	}
	impl AddAssign for OrdF64 {
		fn add_assign(&mut self, rhs: Self) { self.0 += rhs.0; }
	}
	impl Sub for OrdF64 { type Output = OrdF64;
		fn sub(self, rhs: Self) -> Self::Output { OrdF64(self.0 - rhs.0) }
	}
	impl SubAssign for OrdF64 {
		fn sub_assign(&mut self, rhs: Self) { self.0 -= rhs.0; }
	}
	impl Mul for OrdF64 { type Output = OrdF64;
		fn mul(self, rhs: Self) -> Self::Output { OrdF64(self.0 * rhs.0) }
	}
	impl MulAssign for OrdF64 {
		fn mul_assign(&mut self, rhs: Self) { self.0 *= rhs.0; }
	}
	impl Div for OrdF64 { type Output = OrdF64;
		fn div(self, rhs: Self) -> Self::Output { OrdF64(self.0 / rhs.0) }
	}
	impl DivAssign for OrdF64 {
		fn div_assign(&mut self, rhs: Self) { self.0 /= rhs.0; }
	}
	impl Neg for OrdF64 { type Output = OrdF64;
		fn neg(self) -> Self::Output { OrdF64(-self.0) }
	}
} pub use ord_f64::OrdF64;
