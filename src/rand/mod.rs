//! Randomization.

/// A simple PRNG.
/// 
/// Base code was taken from [kiwiyou](https://snippets.kiwiyou.dev/prng).
/// 
/// # Initialization
/// - `new` initializes a new random generator.
/// 
/// # Query
/// - `next(n)` generates a random integer in `[0, n)`.
/// 
/// # Caution
/// `next` panics if `n == 0`.
/// 
/// # Example
/// ```
/// use ps_snippets::rand::rand::*;
/// 
/// let mut rng = Rng32::new();
/// for _ in 0..100 {
///     assert!(rng.next(10) < 10);
/// }
/// ```
#[allow(clippy::module_inception)]
pub mod rand;
