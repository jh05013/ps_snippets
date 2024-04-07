//! Numeric types.

/// [`f64`] but with [`Ord`] trait.
/// 
/// Of course, it assumes the absence of NaN values,
/// which you wouldn't really encounter in PS.
/// 
/// Supports arithmetic, printing, and derefs into `f64`.
/// 
/// # Examples
/// ```
/// # use ps_snippets::number::ord_f64::OrdF64;
/// let a = OrdF64(2.5);
/// let b = OrdF64(3.5);
/// assert_eq!(a + b, OrdF64(6.0));
/// assert_eq!(a.min(b), a); // Ord trait method
/// assert_eq!(a.sin(), 2.5_f64.sin()); // Deref f64 method
/// ```
pub mod ord_f64;

/// Modulo `1000000007` integers.
/// 
/// There is also a `998244353` version: see [`modint_998244353`].
/// For any other prime modulo, copy-paste the source code and change
/// the constant `MOD`.
/// 
/// # `Modint`
/// 
/// ## Initialization
/// - `new(n)` constructs an integer for `n` modulo `1000000007`.
/// - `modint(n)` is a shorthand for `Modint::new(n)`.
/// 
/// ## Operations
/// - Input and output.
/// - Conversion to/from `i32`.
/// - All arithmetic operators, either in-place or not.
///   Division takes $O(\log\ MOD)$.
/// - `inv()` returns the multiplicative inverse in $O(\log\ MOD)$.
/// - Negation.
/// - `pow(n)` returns the `n`-th power in $O(\log n)$.
/// 
/// ## Caution
/// Panics on division by or inversion of 0.
/// 
/// ## Examples
/// ```
/// # use ps_snippets::number::modint_1000000007::modint;
/// let a = modint(12345);
/// let b = modint(54321);
/// assert_eq!(a*b, modint(12345i64 * 54321 % 1000000007));
/// assert_eq!(a * a.inv(), modint(1));
/// assert_eq!(a.pow(3), a*a*a);
/// ```
/// 
/// ## Practice Problems
/// - [BOJ 30155 Crazy Malvika discovers Crazy Fibonacci function](https://www.acmicpc.net/problem/30155) `Add`
/// - [BOJ 11401 이항 계수 3](https://www.acmicpc.net/problem/11401) `Mul`, `Div`
/// - [BOJ 13171 A](https://www.acmicpc.net/problem/13171) `pow`
/// 
/// # `Modfact`
/// A modulo factorial map allowing for a constant-time modulo
/// factorial for small values of `n`.
/// 
/// ## Initialization
/// `new(n)` constructs a modulo factorial map up to `n`.
/// 
/// ## Fields
/// - `fact[i]` is $i!$ modulo 1000000007.
/// - `ifact[i]` is the inverse of $i!$ modulo 1000000007.
/// 
/// ## Queries
/// - `len()` returns the length of `fact`.
/// - `comb(n, r)` returns the binomial coefficient of `n` and `r`,
///    modulo 1000000007, in $O(1)$.
/// 
/// ## Caution
/// `comb` panics if `0 <= r <= n < fact.len()` is not satisfied.
/// 
/// ## Example
/// ```
/// # use ps_snippets::number::modint_1000000007::*;
/// let mf = Modfact::new(10);
/// assert_eq!(mf.fact[5], modint(5*4*3*2*1));
/// assert_eq!(mf.fact[5] * mf.ifact[5], modint(1));
/// assert_eq!(mf.comb(10, 4), modint(10*9*8*7/4/3/2/1));
/// ```
/// 
/// ## Practice Problems
/// - [BOJ 13977 이항 계수와 쿼리](https://www.acmicpc.net/problem/13977)
/// 
pub mod modint_1000000007;

/// See `modint_1000000007`.
pub mod modint_998244353;
