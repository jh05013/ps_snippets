//! TODO document this

const MOD: i32 = 1000000007;
const MODL: i64 = MOD as i64;

use std::ops::*;
use std::fmt::*;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Modint { v: i32 }
pub fn modint(v: i64) -> Modint { Modint::new(v) }

impl Modint {
	pub fn new(v: i64) -> Modint { Self { v: v.rem_euclid(MODL) as i32 } }

	pub fn pow(&self, mut n: u64) -> Self {
		let mut ans = Modint::new(1);
		let mut a = *self;
		while n != 0 {
			if n&1 == 1 { ans*= a; }
			n>>= 1; a*= a;
		}
		ans
	}

	pub fn inv(&self) -> Self {
		assert!(self.v != 0);
		self.pow((MOD-2) as u64)
	}
}

// io
impl std::str::FromStr for Modint {
	type Err = std::num::ParseIntError;
	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		Ok(Self::new(s.parse::<i64>()?))
	}
}
impl Display for Modint {
	fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{}", self.v) }
}
impl From<Modint> for i32 {
	fn from(num: Modint) -> i32 { num.v }
}
impl From<i32> for Modint {
	fn from(num: i32) -> Modint { Modint::new(num as i64) }
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
	fn mul_assign(&mut self, b: Self) {
		self.v = ((self.v as i64) * (b.v as i64) % MODL) as i32;
	}
}
impl Mul for Modint { type Output = Self;
	fn mul(self, b: Self) -> Self { let mut z = self; z*= b; z }
}
impl DivAssign for Modint {
	fn div_assign(&mut self, b: Self) { *self = *self / b; }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Modint { type Output = Self;
	fn div(self, b: Self) -> Self { self * b.inv() }
}
