//! Under construction.

use std::cmp::Reverse;

pub fn min_dnc_opt<T>(
	f: impl Fn(usize, usize) -> Option<T>,
	n: usize,
	m: usize,
) -> Vec<T>
where
T: Clone + Default + Ord,
{
	assert!(m > 0);
	if n == 0 {
		return vec![];
	}
	let mut ans = vec![T::default(); n];
	
	let mut task = vec![(0, n-1, 0, m-1)];
	while let Some((imin, imax, jmin, jmax)) = task.pop() {
		let mid = (imin + imax) / 2;
		let (opt_val, Reverse(opt_j)) = (jmin..=jmax)
			.filter_map(|j| {
				if let Some(val) = f(mid, j) { Some((val, Reverse(j))) }
				else { None }
			})
			.min().unwrap();
		ans[mid] = opt_val;

		if imin+1 <= mid {
			task.push((imin, mid-1, jmin, opt_j));
		}
		if mid+1 <= imax {
			task.push((mid+1, imax, opt_j, jmax));
		}
	}
	ans
}

pub fn min_plus_convolution<T>(convex: &[T], other: &[T]) -> Vec<T>
where
    T: std::ops::Add<Output = T> + Default + Clone + Ord,
{
    let n = convex.len();
    let m = other.len();
	if n == 0 || m == 0 {
		return vec![];
	}

	// check convexity
	assert!(convex.windows(3).all(|w| {
		w[2].clone()+w[0].clone() >= w[1].clone()+w[1].clone()
	}));

	min_dnc_opt(
		|i, j| {
			if i < j || i-j >= n {
				return None;
			}
			Some(convex[i-j].clone() + other[j].clone())
		},
		n+m-1,
		m
	)
}

pub fn max_plus_convolution<T>(concave: &[T], other: &[T]) -> Vec<T>
where
    T: std::ops::Add<Output = T> + Default + Clone + Ord,
{
    let n = concave.len();
    let m = other.len();
	if n == 0 || m == 0 {
		return vec![];
	}

	// check concavity
	assert!(concave.windows(3).all(|w| {
		w[2].clone()+w[0].clone() <= w[1].clone()+w[1].clone()
	}));

	min_dnc_opt(
		|i, j| {
			if i < j || i-j >= n {
				return None;
			}
			Some(Reverse(concave[i-j].clone() + other[j].clone()))
		},
		n+m-1,
		m
	).into_iter().map(|Reverse(v)| v).collect()
}

#[cfg(test)]
mod test {
    use min_plus_convolution;

    fn do_test(convex: &[i64], other: &[i64]) {
        let n = convex.len();
        let m = other.len();
        let ans = min_plus_convolution(convex, other);
        let mut naive = vec![i64::MAX; n + m - 1];
        for (i, x) in convex.iter().enumerate() {
            for (j, y) in other.iter().enumerate() {
                let val = *x + *y;
                naive[i + j] = naive[i + j].min(val);
            }
        }
        assert_eq!(ans, naive);
    }

    #[test]
    fn test_min_plus_convolution() {
        do_test(&[1, 2, 4, 7, 10], &[0, 4, 5, 6, 9, 11, 14, 16, 29]);
    }
}
