//! A barebones library of (geometric) polygons.
//! Use this when you only need simple operations and
//! don't want to import the whole geometry package
//! into your submission.
//!
//! # Examples
//! - `twice_signed_area`
//!   - [eolymp 60 Area of a Polygon](https://basecamp.eolymp.com/en/problems/60)
//!     (and [851](https://basecamp.eolymp.com/en/problems/851),
//!      [2147](https://basecamp.eolymp.com/en/problems/2147),
//!      [6349](https://basecamp.eolymp.com/en/problems/6349))
//!   - [BOJ 2166 다각형의 면적](https://www.acmicpc.net/problem/2166)
//!   - [SWERC 2019F Icebergs](https://www.acmicpc.net/problem/18298)

use std::ops::*;

/// Returns 2 times the signed area of the polygon.
/// The return value is negative if the vertices are
/// given clockwise, and positive if counter-clockwise.
///
/// Note that for polygons with integer coordinates,
/// the area is a half-integer.
/// # Example
/// ```
/// # use polygon_simple::twice_signed_area;
/// assert_eq!(
///     twice_signed_area(&[(0, 0), (2, 0), (2, 2), (0, 2)]),
///     8
/// );
/// assert_eq!(
///     twice_signed_area(&[(0, 0), (0, 2), (2, 2), (2, 0)]),
///     -8
/// );
/// ```
pub fn twice_signed_area<T>(poly: &[(T, T)]) -> T
where
    T: Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + std::iter::Sum<T> + Copy,
{
    let n = poly.len();
    if n <= 2 {
        return T::default();
    }
    poly.windows(2)
        .map(|w| w[0].0 * w[1].1 - w[0].1 * w[1].0)
        .sum::<T>()
        + poly[n - 1].0 * poly[0].1
        - poly[0].0 * poly[n - 1].1
}
