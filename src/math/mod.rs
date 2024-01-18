//! Math.
//! 
//! For prime-related utilities such as Eratosthenes sieve, check out the `prime` module.

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
/// # use ps_snippets::math::modint_1000000007::modint;
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
/// # use ps_snippets::math::modint_1000000007::*;
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

/// Greatest common divisor and least common multiple.
/// - `Gcd` trait supports `a.gcd(b)`, the greatest common divisor.
///   If either a or b is 0, assume their GCD is the other one.
/// - `Lcm` trait supports `a.lcm(b)`, the least common multiple.
///   If either a or b is 0, assume their LCM is 0.
/// 
/// Primitive unsigned integers implement `Gcd` and `Lcm` in
/// $O(\log a + \log b)$, using the
/// [binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm).
/// It is much faster (like 2.5x!) than the well-known Euclidean algorithm.
/// 
/// # Example
/// ```
/// use ps_snippets::math::gcd::*;
/// 
/// assert_eq!(6u32.gcd(4), 2);
/// assert_eq!(6u64.lcm(4), 12);
/// assert_eq!(0u32.gcd(4), 4);
/// assert_eq!(4u32.gcd(0), 4);
/// assert_eq!(0u32.lcm(4), 0);
/// assert_eq!(4u32.lcm(0), 0);
/// ```
/// 
/// # Practice Problems
/// - [eolymp 571 The greatest common divisor](https://www.eolymp.com/en/problems/571) `gcd`
/// - [eolymp 135 LCM](https://www.eolymp.com/en/problems/135) `lcm` (use `u128`)
/// - [PE 5 Smallest Multiple](https://projecteuler.net/problem=5) `lcm`
/// - [BOJ 30449 Reafy 수열](https://www.acmicpc.net/problem/30449) blazingly fast `gcd`,
///   can be used as a performance benchmark.
///   My implementation gives 0.4s with `select_nth_unstable_by`.
pub mod gcd;

/// Barrett reduction, allowing for a fast runtime constant modulo.
/// 
/// # Initialization
/// `new(n)` constructs a Barrett reduction interface modulo `n`.
/// 
/// # Operations
/// - `reduce(x)` returns `x` mod `n`.
/// - `modpow(x, k)` returns `x` to the `k`-th power mod `n` in $O(\log k)$.
/// 
/// # Caution
/// `new` panics if `n == 0`.
/// 
/// # Example
/// ```
/// # use ps_snippets::math::barrett::*;
/// let bar = Barrett::new(1000);
/// assert_eq!(bar.reduce(1234567), 567);
/// assert_eq!(bar.modpow(2, 20), 576); // 1048576
/// ```
/// 
/// # Practice Problems
/// - [BOJ 17466 N! mod P (1)](https://www.acmicpc.net/problem/17466)
/// - [BOJ 17467 N! mod P (2)](https://www.acmicpc.net/problem/17467) (requires Wilson's theorem)
pub mod barrett;

/// Iterator for floor divisions of `n`.
/// 
/// It can be proven that for a non-negative integer $n$, the expression
/// $\lfloor \frac{n}{x} \rfloor$ for positive integer $x$ can have
/// $O(\sqrt{n})$ different values.
/// 
/// # Functions
/// `div_floors(n)` returns an iterator. Each item is of the form
/// `(k, l, r)`, where $k \geq 1$, which denotes that `n/x == k` for
/// `x` in `[l..=r]`. The items are given in the decreasing order of `k`.
/// 
/// Implementation note: only 1 division is carried out per item, which
/// roughly maximizes the speed.
/// 
/// # Example
/// ```
/// # use ps_snippets::math::div_floors::div_floors;
/// /*
/// x    1 2 3 4 5 6 7 8 9 10 11 12 13
/// ------------------------------------
/// N/x 10 5 3 2 2 1 1 1 1  1  0  0  0
/// */
/// let mut df = div_floors(10);
/// assert_eq!(df.next(), Some((10, 1, 1)));
/// assert_eq!(df.next(), Some((5, 2, 2)));
/// assert_eq!(df.next(), Some((3, 3, 3)));
/// assert_eq!(df.next(), Some((2, 4, 5)));
/// assert_eq!(df.next(), Some((1, 6, 10)));
/// assert_eq!(df.next(), None);
/// ```
/// 
/// # Practice Problems
/// - [LC Enumerate Quotients](https://judge.yosupo.jp/problem/enumerate_quotients) 1e12, 108 ms
/// - [BOJ 26056 수열의 합 2](https://www.acmicpc.net/problem/26056) 1e14, twice in 436 ms
pub mod div_floors;
