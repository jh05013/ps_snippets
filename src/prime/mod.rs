//! A collection of prime-related functions and prime sieves.
//! 
//! For describing time complexities, we'll denote
//! $\pi(n) = O(\frac{n}{\log n})$ as the number of primes under $n$.

pub mod prime;

pub mod prime_sqrt;
pub mod basic_sieve;
pub mod segmented_sieve;
