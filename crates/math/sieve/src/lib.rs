//! Eratosthenes' sieve and more.

/// Generates an array of length `n+1` by sieving.
///
/// The process is as follows:
/// 1. Initially, `arr[i] = initial(i)`.
/// 2. For each `p >= 1`:
///    check if `check(p, &arr[p])` holds.
///    If so, for each `q >= p` a multiple of `p`,
///    call `update(&mut arr, p, q)`.
///
/// Check out the source code of [`prime_sieve`],
/// TODO, etc. for examples.
///
/// # Time complexity
/// - `initial` and `check` are invoked `O(n)` times each.
/// - `update` is invoked `O(nlogn)` times,
///   or possibly fewer depending on `check`.
///   For example, [`prime_sieve`] invokes it `O(nloglogn)` times.
pub fn general_sieve<T>(
    n: usize,
    initial: impl Fn(usize) -> T,
    check: impl Fn(usize, &T) -> bool,
    update: impl Fn(&mut [T], usize, usize),
) -> Vec<T> {
    let mut sieve = (0..=n).map(initial).collect::<Vec<_>>();
    for p in 1..=n {
        if !check(p, &sieve[p]) {
            continue;
        }
        for q in (p..=n).step_by(p) {
            update(&mut sieve, p, q);
        }
    }
    sieve
}

/// Returns a [`bool`] vector of length `n+1`
/// s.t `arr[i]` is true iff `i` is a prime.
///
/// 🕒 `O(nloglogn)`.
///
/// # Example
/// ```
/// # use sieve::prime_sieve;
/// assert_eq!(
///     prime_sieve(5),
///     vec![false, false, true, true, false, true]
/// );
/// ```
pub fn prime_sieve(n: usize) -> Vec<bool> {
    general_sieve(
        n,
        |i| (i >= 2),
        |_, &x| x,
        |sieve, p, q| {
            sieve[q] = p == q;
        },
    )
}

/// Returns a [`usize`] vector of length `n+1`
/// representing [Euler's phi function](https://en.wikipedia.org/wiki/Euler%27s_totient_function).
///
/// 🕒 `O(nlogn)`.
///
/// # Example
/// ```
/// # use sieve::phi_sieve;
/// assert_eq!(
///     phi_sieve(10),
///     vec![0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4]
/// );
/// ```
pub fn phi_sieve(n: usize) -> Vec<usize> {
    general_sieve(
        n,
        |i| i,
        |p, x| p > 1 && *x == p,
        |arr, p, q| {
            arr[q] -= arr[q] / p;
        },
    )
}
