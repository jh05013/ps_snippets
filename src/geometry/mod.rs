//! Geometry.

/// A 2D point primitive.
/// 
/// `pnt(x, y)` constructs a point at the coordinate $(x, y)$.
/// 
/// The coordinate type must implement `Coord`, which
/// is a supertrait of common traits and arithmetic operator traits, and
/// contains just a conversion to f64 (for now). Specifically:
/// ```ignore
/// trait Coord:
///     Copy + Debug + Default + PartialEq + PartialOrd
///     + AddAssign + Add<Output=Self> + SubAssign + Sub<Output=Self>
///     + MulAssign + Mul<Output=Self> + DivAssign + Div<Output=Self>
///     + Neg<Output=Self>
/// {
///     fn to_f64(&self) -> f64;
/// }
/// ```
/// `i32`, `i64`, `i128`, and `f64` all implement `Coord`.
/// 
/// # Methods
/// Too many to list; check out the example below and the doc for `Pnt`.
/// 
/// # Examples
/// ```
/// use ps_snippets::geometry::pnt::*;
/// 
/// let a = pnt(0, 0);
/// let b = pnt(2, 2);
/// let c = pnt(0, 4);
/// 
/// // Distances and dot/cross products.
/// assert_eq!(c.sq(), 0*0 + 4*4);
/// assert_eq!(c.abs(), 4.);
/// assert_eq!(b.distsq(c), 2*2 + 2*2);
/// assert_eq!(c.dist(a), 4.);
/// assert_eq!(b.dot(c), 2*0 + 2*4);
/// assert_eq!(b.cross(c), 2*4 - 2*0);
/// 
/// // Angles.
/// assert_eq!(c.arg(), std::f64::consts::PI * 0.5);
/// assert_eq!(a.arg_at(c), std::f64::consts::PI * -0.5);
/// assert_eq!(a.orient(b, c), 2*4 - 2*0);
/// 
/// // Transformation.
/// assert_eq!(b.scale(c, 2), pnt(4, 0));
/// assert_eq!(b.rot90(), pnt(-2, 2));
/// assert_eq!(b.rot90_at(c), pnt(2, 6));
/// assert_eq!(b.flip_at(c), pnt(-2, 6));
/// // TODO scale_to, rot, rot_at
/// 
/// // Arithmetic.
/// // Assigments are also allowed if mutable, e.g. b += c
/// assert_eq!(-b, pnt(-2, -2));
/// assert_eq!(b + c, pnt(2, 6));
/// assert_eq!(b - c, pnt(2, -2));
/// assert_eq!(b * 2, pnt(4, 4));
/// assert_eq!(b / 3, pnt(0, 0)); // i32 division
/// ```
/// 
/// # Practice Problems
/// TODO
pub mod pnt;
