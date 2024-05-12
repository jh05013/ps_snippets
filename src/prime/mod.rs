//! A collection of prime-related functions and prime sieves.
//! 
//! For describing time complexities, we'll denote
//! $\pi(n) = O(\frac{n}{\log n})$ as the number of primes under $n$.

/// Use this module if you just need the prime functionalities,
/// not necessarily blazing fast.
/// 
/// No optimization is applied beyond the most common ones.
/// This is useful in assessing a problem's difficulty during
/// problemsetting.
/// 
/// ## `basic_primes_mod`
/// - `prime_sieve(n)` returns the vector `sieve` of length `n`
/// such that `sieve[i]` is true iff `i` is prime,
/// in $O(n \log \log n)$.
/// - `primes(n)` returns the vector of primes up to (including) `n`
/// in increasing order, in $O(n \log \log n)$.
/// 
/// ## `basic_primality_mod`
/// - `is_prime(n)` returns true iff `n` is a prime. Takes $O(\sqrt n)$
///   if `n` is a prime, and otherwise $O(p)$ where `p` is the smallest
///   prime factor of `n`.
/// - `nth_prime(n)` returns the `n`-th prime counting from 0th
/// (so 0th prime is 2), in $O(n \sqrt n)$.
/// 
/// ## `basic_factor_mod`
/// - `factorize(n)` returns the prime factorization of `n`.
///   It is an iterator of `(prime factor, multiplicity)`, where
///   prime factors are distinct and in increasing order.
///   For `n < 2`, the iterator is empty.
///   Full iteration $O(\sqrt n)$ if `n` is a prime, and otherwise
///   $O(p)$ where `p` is the largest prime factor of `n`.
/// 
/// # Example
/// ```
/// # use ps_snippets::prime::basic_prime::*;
/// assert_eq!(prime_sieve(4), vec![false, false, true, true, false]);
/// assert_eq!(primes(11), vec![2, 3, 5, 7, 11]);
/// assert_eq!(primes(1), vec![]);
/// 
/// assert!(is_prime(11));
/// assert!(!is_prime(0) && !is_prime(1) && !is_prime(9));
/// assert_eq!(nth_prime(0), 2);
/// assert_eq!(nth_prime(4), 11);
/// 
/// let mut factorization = factorize(24); // 2^3 * 3^1
/// assert_eq!(factorization.next(), Some((2, 3)));
/// assert_eq!(factorization.next(), Some((3, 1)));
/// assert_eq!(factorization.next(), None);
/// assert_eq!(factorize(0).next(), None);
/// assert_eq!(factorize(1).next(), None);
/// assert_eq!(factorize(2).next(), Some((2, 1)));
/// ```
/// 
/// # Practice Problems
/// - `basic_primes_mod`
///   - [BOJ 1219 소수 구하기](https://www.acmicpc.net/problem/1929)
///   - [PE 10 Summation of Primes](https://projecteuler.net/problem=10)
///   - [DMOJ alexquiz2](https://dmoj.ca/problem/alexquiz2)
///     (with [`RangeSum`])
/// - `basic_primality_mod`
///   - [Waterloo 2012-7 Next Prime](https://www.acmicpc.net/problem/4134)
///     = [DMOJ bf3](https://dmoj.ca/problem/bf3)
///   - [PE 7 10001st Prime](https://projecteuler.net/problem=7)
/// - `basic_factor_mod`
///   - [PE 3 Largest Prime Factor](https://projecteuler.net/problem=3)
///   - [DMOJ primefactor](https://dmoj.ca/problem/primefactor)
pub mod basic_prime;

// pub mod prime_sqrt;
// pub mod basic_sieve;
// pub mod segmented_sieve;
