pub mod modint_998244353 {
	pub const MOD: i32 = 998_244_353;
	pub const MODL: i64 = 998_244_353;
	pub const MODUL: u64 = 998_244_353;
	pub const MODUS: usize = 998_244_353;
	
	use std::{ops::*, fmt::*};
	
	#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
	pub struct Modint { v: i32 }

	impl Modint {
		pub fn pow(&self, mut n: u64) -> Self {
			let mut ans = Self::from(1);
			let mut a = *self;
			while n != 0 {
				if n&1 == 1 { ans*= a; }
				n>>= 1; a*= a;
			}
			ans
		}
	
		pub fn inv(&self) -> Self {
			assert!(self.v != 0, "Cannot invert 0");
			self.pow(MODUL-2)
		}
	}
	
	// io
	impl std::str::FromStr for Modint {
		type Err = std::num::ParseIntError;
		fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
			Ok(Self::from(s.parse::<i64>()?))
		}
	}
	impl Display for Modint {
		fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{}", self.v) }
	}
	impl From<Modint> for i32 {
		fn from(num: Modint) -> Self { num.v }
	}
	impl From<Modint> for i64 {
		fn from(num: Modint) -> Self { Self::from(num.v) }
	}
	impl From<i32> for Modint {
		fn from(num: i32) -> Self { Self { v: num.rem_euclid(MOD) } }
	}
	#[allow(clippy::as_conversions)]
	impl From<i64> for Modint {
		fn from(num: i64) -> Self { Self { v: num.rem_euclid(MODL) as i32 } }
	}
	#[allow(clippy::as_conversions)]
	impl From<u64> for Modint {
		fn from(num: u64) -> Self { Self { v: num.rem_euclid(MODUL) as i32 } }
	}
	#[allow(clippy::as_conversions)]
	impl From<usize> for Modint {
		fn from(num: usize) -> Self { Self { v: num.rem_euclid(MODUS) as i32 } }
	}
	
	// arithmetic
	impl AddAssign for Modint {
		fn add_assign(&mut self, b: Self) {
			self.v+= b.v; if self.v >= MOD { self.v-= MOD; }
		}
	}
	impl Add for Modint { type Output = Self;
		fn add(self, b: Self) -> Self { let mut z = self; z+= b; z }
	}
	impl Neg for Modint { type Output = Self;
		fn neg(self) -> Self { Self::default() - self }
	}
	impl SubAssign for Modint {
		fn sub_assign(&mut self, b: Self) {
			self.v-= b.v; if self.v < 0 { self.v+= MOD; }
		}
	}
	impl Sub for Modint { type Output = Self;
		fn sub(self, b: Self) -> Self { let mut z = self; z-= b; z }
	}
	impl MulAssign for Modint {
		fn mul_assign(&mut self, b: Self) { *self = *self * b; }
	}
	impl Mul for Modint { type Output = Self;
		fn mul(self, b: Self) -> Self { Self::from(
			i64::from(self) * i64::from(b)
		) }
	}
	impl DivAssign for Modint {
		fn div_assign(&mut self, b: Self) { *self = *self / b; }
	}
	#[allow(clippy::suspicious_arithmetic_impl)]
	impl Div for Modint { type Output = Self;
		fn div(self, b: Self) -> Self { self * b.inv() }
	}
}
pub use modint_998244353::{MOD, MODL, Modint};
