pub mod pnt_mod {
	use std::fmt::Debug;
	use std::ops::*;

	pub trait Coord:
	Copy + Debug + Default + PartialEq + PartialOrd
		+ AddAssign + Add<Output=Self> + SubAssign + Sub<Output=Self>
		+ MulAssign + Mul<Output=Self> + DivAssign + Div<Output=Self>
		+ Neg<Output=Self>
	{
		fn to_f64(&self) -> f64;
	}

	impl Coord for i32 {
		#[inline] fn to_f64(&self) -> f64 { *self as f64 }
	}
	impl Coord for i64 {
		#[inline] fn to_f64(&self) -> f64 { *self as f64 }
	}
	impl Coord for i128 {
		#[inline] fn to_f64(&self) -> f64 { *self as f64 }
	}
	impl Coord for f64 {
		#[inline] fn to_f64(&self) -> f64 { *self }
	}

	/// A 2D point.
	#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Pnt<T: Coord> {
		pub x: T, pub y: T
	}

	#[inline] pub fn pnt<T: Coord>(x: T, y: T) -> Pnt<T> { Pnt{x,y} }

	impl<T: Coord> Pnt<T> {
		/// Squared distance from (0, 0).
		#[inline] pub fn sq(&self) -> T { self.x*self.x + self.y*self.y }
		/// Distance from (0, 0).
		#[inline] pub fn abs(&self) -> f64 { self.sq().to_f64().sqrt() }
		/// Squared distance from `b`.
		#[inline] pub fn distsq(&self, b: Self) -> T { (*self-b).sq() }
		/// Distance from `b`.
		#[inline] pub fn dist(&self, b: Self) -> f64 { self.distsq(b).to_f64().sqrt() }
		/// Dot product with `b`.
		#[inline] pub fn dot(&self, b: Self) -> T { self.x*b.x + self.y*b.y }
		/// Cross product with `b`.
		#[inline] pub fn cross(&self, b: Self) -> T { self.x*b.y - self.y*b.x }

		/// Angle measured from +x axis, ccw.
		#[inline] pub fn arg(&self) -> f64 { self.y.to_f64().atan2(self.x.to_f64()) }
		/// Angle measured from +x direction, centered at `c`, ccw.
		#[inline] pub fn arg_at(&self, c: Self) -> f64 { (*self-c).arg() }
		/// Cross product of `self->b` and `b->c`.
		#[inline] pub fn orient(&self, b: Self, c: Self) -> T { (b-*self).cross(c-*self) }

		/// Resized `f` times, centered at `c`.
		#[inline] pub fn scale(&self, c: Self, f: T) -> Self { c + (*self-c)*f }
		/// Rotated 90 degrees ccw, centered at (0, 0).
		#[inline] pub fn rot90(&self) -> Self { pnt(-self.y, self.x) }
		/// Rotated 90 degrees ccw, centered at `c`.
		#[inline] pub fn rot90_at(&self, c: Self) -> Self { (*self-c).rot90()+c }
		/// Mirror-reflected, centered at `c`.
		#[inline] pub fn flip_at(&self, c: Self) -> Self { c+c-*self }
	}
	impl Pnt<f64> {
		/// Resized to size `size`, centered at `c`.
		#[inline] pub fn scale_to(&self, c: Self, size: f64) -> Self {
			let v = *self-c; c + v*size/v.abs()
		}
		/// Rotated `theta` radians ccw, centered at (0, 0).
		#[inline] pub fn rot(&self, theta: f64) -> Self {
			let (s, c) = theta.sin_cos();
			pnt(self.x*c - self.y*s, self.x*s + self.y*c)
		}
		/// Rotated `theta` radians ccw, centered at `c`.
		#[inline] pub fn rot_at(&self, theta: f64, c: Self) -> Self { (*self-c).rot(theta)+c }
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
pub use pnt_mod::{Pnt, pnt};
