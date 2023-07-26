#[cfg(test)]

#[test]
fn gcd_test() {
	use crate::algo::gcd::*;

	fn gcd_naive(a: i32, b: i32) -> i32 {
		for v in (1..=std::cmp::max(a, b)).rev() {
			if a%v == 0 && b%v == 0 {
				return v;
			}
		}
		unreachable!();
	}

	fn lcm_naive(a: i32, b: i32) -> i32 {
		for v in 1..=b {
			if v*a%b == 0 {
				return v*a;
			}
		}
		unreachable!();
	}

	for a in 1..100 {
		for b in 1..100 {
			assert_eq!(a.gcd(b), gcd_naive(a, b));
			assert_eq!(a.lcm(b), lcm_naive(a, b));
		}
	}
	assert_eq!((123456789012345678u64).gcd(105255401205018735), 193425723);
	assert_eq!((123456789012345678u64).lcm(10), 617283945061728390);
	assert_eq!((987654321u64).lcm(123456789), 13548070123626141);
}
