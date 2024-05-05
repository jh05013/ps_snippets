#[test]
fn div_floors_test() {
	use crate::math::harmonic::*;

	fn div_floors_correct(n: u64) {
		for (v, l, r) in div_floors(n) {
			assert_eq!(v, n / l);
			assert_eq!(v, n / r);
			assert!(l == 1 || v != n / (l-1),
				"wrong left end for {}: ({}, {}, {})", n,v,l,r
			);
			assert!(v != n / (r+1),
				"wrong right end for {}: ({}, {}, {})", n,v,l,r
			);
		}
	}

	fn div_floors_total(n: u64) {
		let mut last_r = 0;
		for (_, l, r) in div_floors(n) {
			assert_eq!(last_r + 1, l);
			last_r = r;
		}
		assert_eq!(last_r, n);
	}

	for n in 0..100 {
		div_floors_correct(n);
		div_floors_total(n);
	}
	for n in 10_000_000_000..10_000_000_050 {
		div_floors_correct(n);
		div_floors_total(n);
	}
}
