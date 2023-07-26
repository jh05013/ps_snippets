#[cfg(test)]

#[test]
fn modint_new_test() {
	fn modint_new_eq(v: i64) {
		use crate::algo::modint::*;
		assert_eq!(i32::from(modint(v)), (v.rem_euclid(1000000007) as i32));
	}

	for v in -100..100 {
		modint_new_eq(v);
	}
	for v in 123456..345678 {
		modint_new_eq(v);
	}
	for v in (1000000007 - 100)..(1000000007 + 100) {
		modint_new_eq(v);
	}
	for v in (10000000070000 - 100)..(10000000070000 + 100) {
		modint_new_eq(v);
	}
	for v in (-1000000007 - 100)..(-1000000007 + 100) {
		modint_new_eq(v);
	}
	for v in (-10000000070000 - 100)..(-10000000070000 + 100) {
		modint_new_eq(v);
	}
}

#[test]
fn modint_arith_test() {
	fn modint_arith_eq(a: i64, b: i64) {
		use crate::algo::modint::*;
		if let Some(ans) = a.checked_add(b) {
			assert_eq!(modint(a) + modint(b), modint(ans));
		}
		if let Some(ans) = a.checked_sub(b) {
			assert_eq!(modint(a) - modint(b), modint(ans));
		}
		if let Some(ans) = a.checked_mul(b) {
			assert_eq!(modint(a) * modint(b), modint(ans));
		}
		if b % 1000000007 != 0 {
			assert_eq!(modint(a) / modint(b) * modint(b), modint(a));
		}
	}

	fn modint_assign_eq(a: i64, b: i64) {
		use crate::algo::modint::*;
		let mut num = modint(a);
		num += modint(b);
		assert_eq!(num, modint(a) + modint(b));
		let mut num = modint(a);
		num -= modint(b);
		assert_eq!(num, modint(a) - modint(b));
		let mut num = modint(a);
		num *= modint(b);
		assert_eq!(num, modint(a) * modint(b));
		if b % 1000000007 != 0 {
			let mut num = modint(a);
			num /= modint(b);
			assert_eq!(num, modint(a) / modint(b));
		}
	}

	let vals: [i64; 26] = [
		-3, -2, -1, 0, 1, 2, 3,
		123, 12345, 218472, 64282944, 38157193873, 38745891275197,
		-123, -12345, -218472, -64282944, -38157193873, -38745891275197,
		1000000007-3, 1000000007-2, 1000000007-1, 1000000007,
		1000000007+1, 1000000007+2, 1000000007+3
	];
	for a in vals {
		for b in vals{
			modint_arith_eq(a, b);
			modint_assign_eq(a, b);
		}
	}
}

#[test]
#[should_panic]
fn modint_div0_test() {
	use crate::algo::modint::*;
	let _ = modint(1) / modint(0);
}
