//! Integer factorization.
//!
//! Uses [Pollard's rho algorithm](https://en.wikipedia.org/wiki/Pollard%27s_rho_algorithm).
//! Tested against inputs that break when using a fixed
//! PRNG; this implementation, of course, uses different
//! PRNGs when one fails. See
//! [this blog](https://lpha-z.hatenablog.com/entry/2023/01/15/231500)
//! for more information.
//!
//! # Examples
//! - [LC Factorize](https://judge.yosupo.jp/problem/factorize)
//! ```ignore
//! for _ in 0..oj.usize() {
//!     let pfac = factorize(oj.u64());
//!     oj.write(pfac.len()).sp();
//!     for p in pfac {
//!         oj.write(p).sp();
//!     }
//!     oj.ln();
//! }
//! ```
//! - [BOJ 4149 큰 수 소인수분해](https://www.acmicpc.net/problem/4149)

extern crate gcd;
extern crate primality;
use gcd::Gcd;
use primality::is_prime;

// TODO: montgomery reduction
fn f(a: u64, step: u64, m: u64) -> u64 {
    use std::convert::TryInto;
    ((u128::from(a).pow(2) + u128::from(step)) % u128::from(m))
        .try_into()
        .unwrap()
}

/// If `n` is composite, returns `Some(d)` where
/// `d` is a non-trivial divisor of `n`.
/// Otherwise, returns `None`.
///
/// 🕒 Maybe `O(n^1/4)`.
///
/// # Example
/// ```
/// # use factorization::find_nontrivial_divisor;
/// let d = find_nontrivial_divisor(105).unwrap(); // 3, 5, 15, ...
/// assert!(1 < d && d < 105 && 105 % d == 0);
/// assert!(find_nontrivial_divisor(17).is_none());
/// ```
#[must_use]
pub fn find_nontrivial_divisor(n: u64) -> Option<u64> {
    if n <= 1 || is_prime(n) {
        return None;
    }
    if n % 2 == 0 {
        return Some(2);
    }
    for step in 1.. {
        let mut x = step;
        let mut y = f(x, step, n);
        loop {
            let g = x.abs_diff(y).gcd(n);
            if g == 0 || g == n {
                break;
            }
            if g != 1 {
                return Some(g);
            }
            x = f(x, step, n);
            y = f(f(y, step, n), step, n);
        }
    }
    unreachable!()
}

/// Returns the factorization of `n` as the sorted list
/// of prime factors. If `n <= 1`, returns `vec![]`.
///
/// 🕒 Maybe `O(n^1/4)`.
///
/// # Example
/// ```
/// # use factorization::factorize;
/// assert_eq!(factorize(12), vec![2, 2, 3]);
/// assert_eq!(factorize(17), vec![17]);
/// assert_eq!(factorize(1), vec![]);
/// ```
#[must_use]
pub fn factorize(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }

    let mut ans = vec![];
    let mut stack = vec![n];
    while let Some(n) = stack.pop() {
        if let Some(d) = find_nontrivial_divisor(n) {
            stack.extend([d, n / d].iter());
        } else {
            ans.push(n);
        }
    }
    ans.sort_unstable();
    ans
}

/// Returns the factorization of `n` as the sorted list
/// of (prime factor, exponent). If `n <= 1`, returns `vec![]`.
///
/// 🕒 Maybe `O(n^1/4)`.
///
/// # Example
/// ```
/// # use factorization::factorize_grouped;
/// assert_eq!(factorize_grouped(12), vec![(2, 2), (3, 1)]);
/// assert_eq!(factorize_grouped(17), vec![(17, 1)]);
/// assert_eq!(factorize_grouped(1), vec![]);
/// ```
#[must_use]
pub fn factorize_grouped(n: u64) -> Vec<(u64, usize)> {
    if n <= 1 {
        return vec![];
    }

    // `chunk_by` would've worked here but that breaks MSRV
    let fac = factorize(n);
    let mut ans = vec![];
    let mut i = 0;
    for (j, x) in fac.iter().enumerate() {
        if fac[i] != *x {
            ans.push((fac[i], j - i));
            i = j;
        }
    }
    ans.push((fac[i], fac.len() - i));
    ans
}

#[cfg(test)]
mod test {
    use primality::is_prime;

    use crate::{factorize, factorize_grouped, find_nontrivial_divisor};

    #[test]
    fn test_find_nontrivial_divisor() {
        fn do_test(n: u64) {
            if let Some(d) = find_nontrivial_divisor(n) {
                assert!(1 < d && d < n && n % d == 0, "{} vs {}", d, n);
            } else {
                assert!(n <= 1 || is_prime(n));
            }
        }

        for n in 1..=100000 {
            do_test(n);
        }
        for n in u64::MAX - 10000..=u64::MAX {
            do_test(n);
        }
    }

    #[test]
    fn test_factorize() {
        fn do_test(n: u64) {
            let fac = factorize(n);
            for w in fac.windows(2) {
                assert!(w[0] <= w[1]);
            }
            for p in &fac {
                assert!(1 < *p && *p <= n, "{} {}", p, n);
                assert!(is_prime(*p), "{}", p);
            }
            assert_eq!(fac.into_iter().product::<u64>(), n);
        }

        assert_eq!(factorize(0), vec![]);
        for n in 1..=10000 {
            do_test(n);
        }
        for n in u64::MAX - 1000..=u64::MAX {
            do_test(n);
        }
    }

    // https://github.com/yosupo06/library-checker-problems/tree/master/number_theory/factorize/gen
    // https://lpha-z.hatenablog.com/entry/2023/01/15/231500
    #[test]
    fn test_factorize_counterexamples() {
        fn do_test(n: u64) {
            let fac = factorize(n);
            for p in &fac {
                assert!(1 < *p && *p <= n, "{} {}", p, n);
                assert!(is_prime(*p), "{}", p);
            }
            assert_eq!(fac.into_iter().product::<u64>(), n);
        }

        do_test(273772559);
        do_test(4295098369);
        do_test(7482809861);
        do_test(124376107291);
        do_test(7816550168663);
        do_test(814483663644399613);
        do_test(999381247093216751);
    }

    #[test]
    fn test_factorize_grouped() {
        fn do_test(n: u64) {
            let fac = factorize_grouped(n);
            for w in fac.windows(2) {
                assert!(w[0] <= w[1]);
            }
            for (p, _) in &fac {
                assert!(1 < *p && *p <= n, "{} {}", p, n);
                assert!(is_prime(*p), "{}", p);
            }
            assert_eq!(
                fac.into_iter()
                    .map(|(p, e)| p.pow(e as u32))
                    .product::<u64>(),
                n
            );
        }

        assert_eq!(factorize(0), vec![]);
        for n in 1..=1000 {
            do_test(n);
        }
        for n in u64::MAX - 100..=u64::MAX {
            do_test(n);
        }
    }
}
