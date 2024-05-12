#[cfg(test)]

#[test]
fn basic_prime_test() {
	use crate::prime::basic_prime::*;

	// Sieve test
	let p100 = vec![2, 3, 5, 7, 11,
		13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
		53, 59, 61, 67, 71, 73, 79, 83, 89, 97
	];
	let sieve = prime_sieve(100);
	for p in 0..=100 {
		assert_eq!(sieve[p], p100.contains(&p));
	}
	assert_eq!(sieve.len(), 101);

	let real_primes = |n: usize| {
		p100.iter().copied().filter(|x| *x <= n)
			.collect::<Vec<_>>()
	};
	for n in 0..=100 {
		assert_eq!(primes(n), real_primes(n));
	}
	assert_eq!(
		primes(2000000).into_iter().sum::<usize>(),
		142_913_828_922
	); // Project Euler 10

	// Primality test
	for n in 0..=100 {
		assert_eq!(is_prime(n), p100.contains(&(n as usize)));
	}
	assert!(is_prime(100_000_000_000_031));
	assert!(!is_prime(100_000_980_001_501)); // 10000019 * 10000079

	// n-th prime test
	for n in 0..p100.len() {
		assert_eq!(nth_prime(n), p100[n] as u64);
	}
	assert_eq!(nth_prime(10000), 104743); // Project Euler 7

	// factor test
	assert_eq!(factorize(0).collect::<Vec<_>>(), vec![]);
	assert_eq!(factorize(1).collect::<Vec<_>>(), vec![]);
	assert_eq!(factorize(2).collect::<Vec<_>>(), vec![(2, 1)]);
	assert_eq!(factorize(9).collect::<Vec<_>>(), vec![(3, 2)]);
	assert_eq!(factorize(24).collect::<Vec<_>>(), vec![(2, 3), (3, 1)]);
	assert_eq!(factorize(32).collect::<Vec<_>>(), vec![(2, 5)]);
	assert_eq!(factorize(600_851_475_143)
		.last().unwrap().0, 6857); // Project Euler 3
}

#[test]
fn basic_factor_test() {
	use crate::prime::basic_prime::*;

	fn verify(n: u64, fact: Vec<(u64, usize)>) {
		// each pair is valid
		fact.iter().for_each(|(p, d)| {
			assert!(is_prime(*p));
			assert!(*d > 0);
		});

		// prime factors are sorted
		fact.windows(2)
			.for_each(|f| {
				assert!(f[0].0 < f[1].0);
			});
		
		// product is n
		assert_eq!(
			std::cmp::max(1, n),
			fact.iter().map(|(p, d)| p.pow(*d as u32)).product::<u64>()
		);
	}

	for n in 0..1000 {
		verify(n, factorize(n).collect::<Vec<_>>());
	}
	for n in 1_000_000_000_000..1_000_000_000_100 {
		verify(n, factorize(n).collect::<Vec<_>>());
	}
}

