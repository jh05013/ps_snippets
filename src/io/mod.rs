//! Easier IO interface specifically designed for online judges.
//! 
//! # Caution
//! The following **unique IO rules** apply to all IO wrappers:
//! - Do not create more than one instance of IO wrappers.
//! - Do not directly read from stdin or write to stdout once an IO wrapper is created.

/// IO wrapper that reads the whole input, ignoring all whitespaces.
/// 
/// To save memory, use TODO. To read by lines, use TODO.
/// 
/// # Read
/// - `try_read::<T>()` tries to read a value of type `T`, returning [`Result`].
///   It fails on EOF or a token that cannot be parsed as `T`.
/// - `read::<T>()` reads a value of type `T`.
///   - `i32()`, `i64()`, `u32()`, `u64()`, `f64()`, and `string()` are
///     the specializations of `read`.
/// - `read_vec::<T>(n)` reads `n` values of type `T` into a [`Vec`].
/// 
/// # Write
/// - `write(v)` prints `v`.
/// - `debug(v)` prints `v` in [`Debug`] form.
/// - `sp()` prints `' '`.
/// - `ln()` prints `'\n'`.
/// - `quit(v)` prints `v` and quits.
/// 
/// Write methods can be chained, e.g. `oj.write(1).sp().write(2)` prints `1 2`.
/// 
/// # Caution
/// - Unique IO rules apply.
/// - `read` and `read_vec` panics on failure.
/// 
/// # Example
/// The following code solves [BOJ 15552](https://www.acmicpc.net/problem/15552)
/// and [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb):
/// ```no_run
/// use ps_snippets::io::oj_default::*;
/// 
/// let mut oj = OJ::new();
/// for _ in 0..oj.read() {
///     let (a, b) = (oj.i64(), oj.i64());
///     oj.write(a+b).ln();
/// }
/// ```
/// 
/// # Practice Problems
/// - [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb) = [BOJ 15552 빠른 A+B](https://www.acmicpc.net/problem/15552) `read`, `write`
/// - [BOJ 10951 A+B - 4](https://www.acmicpc.net/problem/10951) `try_read`
/// - [BOJ 2750 수 정렬하기](https://www.acmicpc.net/problem/2750) `read_vec`
pub mod oj_default;
