//! Randomization.

/// A simple PRNG.
/// 
/// Base code was taken from [kiwiyou](https://snippets.kiwiyou.dev/prng).
/// 
/// # Initialization
/// - `new()` initializes a new unseeded random generator.
/// - `seeded(seed)` initializes a new seeded random generator.
/// 
/// # Query
/// - `next_u32(n)` generates a random integer in `[0, n)` where `n: u32`.
///   It is faster than `next_u64(n)` below.
/// - `next_u64(n)` generates a random integer in `[0, n)` where `n: u64`.
/// - `shuffle(vals)` shuffles the slice `vals`.
///   It uses the [Fisher-Yates algorithm](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle).
/// 
/// # Caution
/// - `next_*` panics if `n == 0`.
/// - `shuffle` panics if `vals` has $2^{32}$ or more values.
///   (Why would you need that long list for PS??)
/// 
/// # Example
/// ```
/// use ps_snippets::rand::rand::*;
/// 
/// let mut rng = Rng::new();
/// let mut vals = vec![1,2,3,4,5];
/// for _ in 0..100 {
///     assert!(rng.next_u32(10) < 10);
///     assert!(rng.next_u64(10) < 10);
///     rng.shuffle(&mut vals);
///     vals.sort();
///     assert_eq!(vals, vec![1,2,3,4,5]);
/// }
/// 
/// // Seeded RNG gives deterministic outputs.
/// let v = Rng::seeded(1234).next_u64(u64::MAX);
/// for _ in 0..100 {
///     assert_eq!(v, Rng::seeded(1234).next_u64(u64::MAX));
/// }
/// ```
#[allow(clippy::module_inception)]
pub mod rand;
