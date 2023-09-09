#[cfg(test)]

#[test]
fn binary_search_test() {
	use crate::algo::binary_search::*;

	fn f100(n: i32) -> Option<i32> {
		if n <= 100 { Some(n + 1234) }
		else { None }
	}
	fn g100(n: f64) -> Option<f64> {
		if n >= 100. { Some(n) }
		else { None }
	}

	assert_eq!(binary_search(f100, -1_000_000_000, 1_000_000_000).unwrap(), (100, 1334));
	assert_eq!(binary_search(f100, 1_000_000_000, -1_000_000_000).unwrap(), (100, 1334));
	let f = binary_search(g100, -1_000_000_000_000., 1_000_000_000_000.).unwrap().0;
	assert!((f - 100.).abs() <= 1e-12);
	let f = binary_search(g100, 1_000_000_000_000., -1_000_000_000_000.).unwrap().0;
	assert!((f - 100.).abs() <= 1e-12);
	for no in 101..200 {
		for yes in 0..101 {
			assert_eq!(binary_search(f100, yes, no).unwrap().0, 100);
			assert_eq!(binary_search(f100, no, yes).unwrap().0, 100);
		}
	}
}

#[test]
#[should_panic]
fn binary_search_err_test() {
	use crate::algo::binary_search::*;

	fn f100(n: i32) -> Option<i32> {
		if n <= 100 { Some(n + 1234) }
		else { None }
	}

	for yes1 in 0..101 {
		for yes2 in 0..101 {
			assert_eq!(binary_search(f100, yes1, yes2), Err((Some(yes1+1234), Some(yes1+1234))));
		}
	}
	for no1 in 101..200 {
		for no2 in 101..200 {
			assert_eq!(binary_search(f100, no1, no2), Err((None, None)));
		}
	}
}
