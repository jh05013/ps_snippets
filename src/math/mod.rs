//! Math.
//! 
//! For prime-related utilities such as Eratosthenes sieve, check out the `prime` module.

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

/// Iterator for division values of `n`.
/// 
/// It can be proven that for a non-negative integer $n$, the expression
/// $\lfloor \frac{n}{x} \rfloor$ for positive integer $x$ can have
/// $O(\sqrt{n})$ different values.
/// 
/// # Functions
/// - `div_floors(n)` returns an iterator. Each item is of the form
///    `(k, l, r)`, where $k \geq 1$, which denotes that `floor(n/x) == k` for
///    `x` in `[l..=r]`. The items are given in the decreasing order of `k`
///    from `n` to `1`.
/// - `div_ceils(n)` is the same, but `ceil(n/x) == k`, and from `n` to `2`.
/// 
/// Implementation note: only 1 division is carried out per item, which
/// roughly maximizes the speed.
/// 
/// # Example
/// ```
/// # use ps_snippets::math::harmonic::div_floors;
/// # use ps_snippets::math::harmonic::div_ceils;
/// /*
/// x           1 2 3 4 5 6 7 8 9 10 11 12 13
/// ------------------------------------
/// floor(N/x) 10 5 3 2 2 1 1 1 1  1  0  0  0
/// */
/// let mut df = div_floors(10);
/// assert_eq!(df.next(), Some((10, 1, 1)));
/// assert_eq!(df.next(), Some((5, 2, 2)));
/// assert_eq!(df.next(), Some((3, 3, 3)));
/// assert_eq!(df.next(), Some((2, 4, 5)));
/// assert_eq!(df.next(), Some((1, 6, 10)));
/// assert_eq!(df.next(), None);
/// 
/// /*
/// x          1 2 3 4 5 6 7 8 9 10 11 12 13
/// ------------------------------------
/// ceil(N/x) 10 5 4 3 2 2 2 2 2  1  1  1  1
/// */
/// let mut df = div_ceils(10);
/// assert_eq!(df.next(), Some((10, 1, 1)));
/// assert_eq!(df.next(), Some((5, 2, 2)));
/// assert_eq!(df.next(), Some((4, 3, 3)));
/// assert_eq!(df.next(), Some((3, 4, 4)));
/// assert_eq!(df.next(), Some((2, 5, 9)));
/// assert_eq!(df.next(), None);
/// ```
/// 
/// # Practice Problems
/// - [LC Enumerate Quotients](https://judge.yosupo.jp/problem/enumerate_quotients) 1e12, 108 ms, floor
/// - [BOJ 26056 수열의 합 2](https://www.acmicpc.net/problem/26056) 1e14, twice in 436 ms, floor
/// - [BOJ 15897 잘못 구현한 에라토스테네스의 체](https://www.acmicpc.net/problem/15897) ceil
pub mod harmonic;
