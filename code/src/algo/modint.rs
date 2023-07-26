pub mod modint {
	const MOD: i32 = 1000000007;
	const MODL: i64 = MOD as i64;

	use std::ops::*;
	use std::fmt::*;

	#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
	pub struct Modint { v: i32 }
	
	impl Modint {
		pub fn new(v: i64) -> Modint { Self { v: v.rem_euclid(MODL) as i32 } }

		pub fn pow(&self, mut n: u64) -> Self {
			let mut ans = Modint::new(1);
			let mut a = self.clone();
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

	// arithmetic
	impl AddAssign for Modint {
		fn add_assign(&mut self, b: Modint) {
			self.v+= b.v; if self.v >= MOD { self.v-= MOD; }
		}
	}
	impl Add for Modint { type Output = Modint;
		fn add(self, b: Modint) -> Modint { let mut z = self; z+= b; z }
	}
	impl SubAssign for Modint {
		fn sub_assign(&mut self, b: Modint) {
			self.v-= b.v; if self.v < 0 { self.v+= MOD; }
		}
	}
	impl Sub for Modint { type Output = Modint;
		fn sub(self, b: Modint) -> Modint { let mut z = self; z-= b; z }
	}
	impl MulAssign for Modint {
		fn mul_assign(&mut self, b: Modint) {
			self.v = ((self.v as i64) * (b.v as i64) % MODL) as i32;
		}
	}
	impl Mul for Modint { type Output = Modint;
		fn mul(self, b: Modint) -> Modint { let mut z = self; z*= b; z }
	}
	impl DivAssign for Modint {
		fn div_assign(&mut self, b: Modint) { *self = *self / b; }
	}
	impl Div for Modint { type Output = Modint;
		fn div(self, b: Modint) -> Modint { self * b.inv() }
	}
} use modint::*;
pub fn modint(v: i64) -> Modint { Modint::new(v) }
