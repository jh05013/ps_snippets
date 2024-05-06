#[cfg(test)]

#[test]
fn basic_sieve_discard_test() {
	use crate::prime::basic_sieve_discard::*;

	let p100 = vec![2, 3, 5, 7, 11,
		13, 17, 19, 23, 29,
		31, 37, 41, 43, 47,
		53, 59, 61, 67, 71,
		73, 79, 83, 89, 97
	];

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
	);
}

/*
#[test]
fn prime_sqrt_test() {
	use crate::math::prime_sqrt::*;

	let p100 = vec![2, 3, 5, 7, 11,
		13, 17, 19, 23, 29,
		31, 37, 41, 43, 47,
		53, 59, 61, 67, 71,
		73, 79, 83, 89, 97
	];

	// is_prime
	let v = (0..100).filter(|n| is_prime(*n)).collect::<Vec<_>>();
	assert_eq!(v, p100.clone());
	assert!(is_prime(100_000_000_000_031));
	assert!(!is_prime(100_000_980_001_501)); // 10000019 * 10000079

	// nth_prime
	for i in 1..=25 {
		assert_eq!(nth_prime(i), p100[i-1]);
	}
	assert_eq!(nth_prime(10000), 104729);

	// next_prime
	assert_eq!(next_prime(0), 2);
	assert_eq!(next_prime(1), 2);
	for i in 0..24 {
		for n in p100[i]..p100[i+1] {
			assert_eq!(next_prime(n), p100[i+1]);
		}
	}
}
*/

/*
#[test]
fn factorization_sqrt_test() {
	use crate::math::prime_sqrt::*;

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

#[test]
#[should_panic]
fn zeroth_prime_sqrt_test() {
	use crate::math::prime_sqrt::*;
	nth_prime(0);
}
*/
