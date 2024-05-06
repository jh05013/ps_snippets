//! A collection of prime-related functions and prime sieves.
//! 
//! For describing time complexities, we'll denote
//! $\pi(n) = O(\frac{n}{\log n})$ as the number of primes under $n$.

/// Use this module if you just need the list of primes,
/// not necessarily blazing fast, and nothing else.
/// 
/// `primes(n)` returns the vector of primes up to `n`
/// in increasing order, in $O(n \log \log n)$.
/// 
/// # Example
/// ```
/// # use ps_snippets::prime::basic_sieve_discard::*;
/// assert_eq!(primes(11), vec![2, 3, 5, 7, 11]);
/// assert_eq!(primes(1), vec![]);
/// ```
/// 
/// # Test Problems
/// - [BOJ 1219 소수 구하기](https://www.acmicpc.net/problem/1929)
/// - [PE 10 Summation of Primes](https://projecteuler.net/problem=10)
pub mod basic_sieve_discard;

// pub mod prime_sqrt;
// pub mod basic_sieve;
// pub mod segmented_sieve;
