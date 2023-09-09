#[cfg(test)]

#[test]
fn div_floors_test() {
	fn div_floors_correct(n: u64) {
		use crate::algo::div_floors::*;
		for (v, l, r) in div_floors(n) {
			assert_eq!(v, n / l);
			assert_eq!(v, n / r);
			assert!(l == 1 || v != n / (l-1));
			assert!(v != n / (r+1));
		}
	}

	fn div_floors_total(n: u64) {
		use crate::algo::div_floors::*;
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
}
