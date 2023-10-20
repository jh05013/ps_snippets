pub mod geom_point {
	use std::fmt::Debug;
	use std::ops::*;

	pub trait Coord:
	Copy + Debug + Default + PartialEq + Eq + PartialOrd
		+ AddAssign + Add<Output=Self> + SubAssign + Sub<Output=Self>
		+ MulAssign + Mul<Output=Self> + DivAssign + Div<Output=Self>
		+ Neg<Output=Self>
	{
		fn to_f64(&self) -> f64;
	}

	impl Coord for i64 {
		#[inline] fn to_f64(&self) -> f64 { *self as f64 }
	}

	#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Pnt<T: Coord> {
		pub x: T, pub y: T
	}

	#[inline] pub fn pnt<T: Coord>(x: T, y: T) -> Pnt<T> { Pnt{x,y} }
	impl<T: Coord> Pnt<T> {
		// basic stuff
		#[inline] pub fn sq(&self) -> T { self.x*self.x + self.y*self.y }
		#[inline] pub fn abs(&self) -> f64 { self.sq().to_f64().sqrt() }
		#[inline] pub fn dot(&self, b: Self) -> T { self.x*b.x + self.y*b.y }
		#[inline] pub fn cross(&self, b: Self) -> T { self.x*b.y - self.y*b.x }

		// orientation
		#[inline] pub fn orient(&self, b: Self, c: Self) -> T { (b-*self).cross(c-*self) }

		// transformation
		#[inline] pub fn scale(&self, c: Self, f: T) -> Self { c + (*self-c)*f }
		#[inline] pub fn rot90(&self) -> Self { pnt(-self.y, self.x) }
		#[inline] pub fn rot90_at(&self, c: Self) -> Self { (*self-c).rot90() + c }
	}

	// ops
	impl<T: Coord> Neg for Pnt<T> { type Output = Pnt<T>;
		fn neg(self) -> Self::Output { pnt(-self.x, -self.y) }
	}
	impl<T: Coord> AddAssign for Pnt<T> {
		fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; }
	}
	impl<T: Coord> Add for Pnt<T> { type Output = Pnt<T>;
		fn add(self, rhs: Self) -> Self::Output { pnt(self.x+rhs.x, self.y+rhs.y) }
	}
	impl<T: Coord> SubAssign for Pnt<T> {
		fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; }
	}
	impl<T: Coord> Sub for Pnt<T> { type Output = Pnt<T>;
		fn sub(self, rhs: Self) -> Self::Output { pnt(self.x-rhs.x, self.y-rhs.y) }
	}
	impl<T: Coord> MulAssign<T> for Pnt<T> {
		fn mul_assign(&mut self, rhs: T) { self.x *= rhs; self.y *= rhs; }
	}
	impl<T: Coord> Mul<T> for Pnt<T> { type Output = Pnt<T>;
		fn mul(self, rhs: T) -> Self::Output { pnt(self.x*rhs, self.y*rhs) }
	}
	impl<T: Coord> DivAssign<T> for Pnt<T> {
		fn div_assign(&mut self, rhs: T) { self.x /= rhs; self.y /= rhs; }
	}
	impl<T: Coord> Div<T> for Pnt<T> { type Output = Pnt<T>;
		fn div(self, rhs: T) -> Self::Output { pnt(self.x/rhs, self.y/rhs) }
	}
}
pub use geom_point::{Pnt, pnt};
