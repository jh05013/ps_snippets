#[test]
fn test_barrett() {
	use crate::math::barrett::Barrett;
	use crate::rand::rand::Rng;

	fn test_barrett_single(v: u64) {
		let bar = Barrett::new(v);
		for i in 0..100 {
			assert_eq!(bar.reduce(i), i%v);
		}
		for i in u64::MAX-100..u64::MAX {
			assert_eq!(bar.reduce(i), i%v);
		}
		let mut rng = Rng::seeded(1234);
		for _ in 0..100 {
			let i = rng.next_u64(u64::MAX);
			assert_eq!(bar.reduce(i), i%v);
		}
	}

	for v in 1..100 { test_barrett_single(v); }
	for v in u64::MAX-100..u64::MAX { test_barrett_single(v); }
	let mut rng = Rng::seeded(12345678);
	for _ in 0..100 {
		let v = rng.next_u64(u64::MAX);
		test_barrett_single(v);
	}
}

#[test]
fn test_barrett_pow() {
	use crate::math::barrett::Barrett;

	fn test_barret_pow_single(m: u64, base: u64, rep: u64) {
		let bar = Barrett::new(m);
		let mut v = 1;
		for k in 0..rep {
			assert_eq!(bar.modpow(base, k), v);
			v = v * base % m;
		}
	}
	
	test_barret_pow_single(1_000_000, 47, 100);
	test_barret_pow_single(1_000_000_000, 10, 30);
}
