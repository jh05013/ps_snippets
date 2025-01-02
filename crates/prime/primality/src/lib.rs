//! Primality test.
//!
//! # Examples
//! - [LC Primality Test](https://judge.yosupo.jp/problem/primality_test)
//! ```ignore
//! for _ in 0..oj.usize() {
//!     if is_prime(oj.u64()) {
//!         oj.write("Yes").ln();
//!     }
//!     else {
//!         oj.write("No").ln();
//!     }
//! }
//! ```
//! - [DMOJ Next Prime (Hard)](https://dmoj.ca/problem/bf3hard)

// TODO: montgomery reduction
pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    use std::convert::TryInto;
    (u128::from(a) * u128::from(b) % u128::from(m))
        .try_into()
        .unwrap()
}

/// `a^n mod m`.
pub fn mod_pow(mut a: u64, mut n: u64, m: u64) -> u64 {
    let mut ans = 1;
    while n != 0 {
        if n & 1 == 1 {
            ans = mod_mul(ans, a, m);
        }
        n >>= 1;
        a = mod_mul(a, a, m);
    }
    ans
}

/// <https://en.wikipedia.org/wiki/Strong_pseudoprime>
const BASES: [u64; 7] = [2, 325, 9375, 28178, 450_775, 9_780_504, 1_795_265_022];

/// Returns `true` iff `n` is a prime.
///
/// Uses [Miller-Rabin test](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
/// with [7 witnesses](https://en.wikipedia.org/wiki/Strong_pseudoprime#Examples).
///
/// 🕒 `O(log^2 n)`.
///
/// # Example
/// ```
/// # use primality::is_prime;
/// assert!(is_prime(17));
/// assert!(!is_prime(100));
/// ```
#[must_use]
pub fn is_prime(n: u64) -> bool {
    if n <= 2 || n % 2 == 0 {
        return n == 2;
    }

    // n-1 = 2^s d
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    for a in BASES {
        let a = a % n;
        if a <= 1 {
            continue;
        }

        let mut x = mod_pow(a, d, n);
        let mut y = 0;
        for _ in 0..s {
            y = mod_mul(x, x, n);
            if y == 1 && x != 1 && x != n - 1 {
                return false;
            }
            x = y;
        }
        if y != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::is_prime;

    #[test]
    fn test_primality_small() {
        const N: usize = 100000;

        // Eratosthenes sieve
        let mut sieve = vec![true; N + 1];
        sieve[0] = false;
        sieve[1] = false;
        for p in 2..=N {
            if !sieve[p] {
                assert!(!is_prime(p as u64), "{}", p);
                continue;
            } else {
                assert!(is_prime(p as u64), "{}", p);
            }
            for q in (p * p..=N).step_by(p) {
                sieve[q] = false;
            }
        }
    }

    #[test]
    fn test_primality_large() {
        let mut n = 18446744073709551557u64;
        assert!(is_prime(n));
        while n < u64::MAX {
            n += 1;
            assert!(!is_prime(n));
        }
    }

    /// https://github.com/yosupo06/library-checker-problems/issues/996
    #[test]
    fn test_primality_pseudoprimes() {
        for n in [
            291831u64,
            341531u64,
            25326001u64,
            885594169u64,
            1050535501u64,
            273919523041u64,
            350269456337u64,
            47636622961201u64,
            55245642489451u64,
            3770579582154547u64,
            7999252175582851u64,
            585226005592931977u64,
            3825123056546413051u64,
        ] {
            assert!(!is_prime(n));
        }
    }
}
