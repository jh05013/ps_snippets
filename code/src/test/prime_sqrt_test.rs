#[cfg(test)]

#[test]
fn prime_sqrt_test() {
	use crate::algo::prime_sqrt::*;

	let p100 = (0..=100u64)
		.filter(|n| is_prime(*n))
		.collect::<Vec<_>>();
	assert_eq!(
		p100,
		vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
			43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
	);
	assert!(is_prime(100_000_000_000_031));
	assert!(!is_prime(100_000_980_001_501)); // 10000019 * 10000079
}
