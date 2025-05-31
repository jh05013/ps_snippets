//! [Longest increasing subsequence](https://en.wikipedia.org/wiki/Longest_increasing_subsequence).
//!
//! Uses an `O(n logn)` binary search algorithm.
//!
//! Returns the list of indices, so to recover the subsequence,
//! use `seq[lis[0]], seq[lis[1]], ...`
//!
//! # Example
//! - [LC Longest Increasing Subsequence](https://judge.yosupo.jp/problem/longest_increasing_subsequence)
//! ```ignore
//! let n = oj.usize();
//! let seq = oj.vec::<u32>(n);
//! let lis = lis(&seq);
//! oj.write(lis.len()).ln();
//! for x in lis {
//!     oj.write(x).sp();
//! }
//! ```
//!
//! To find a decreasing subsequence, use one of:
//! - `lis_by(&seq, |x, y| x.cmp(y).reverse())`
//! - `lis_by_key(&seq, |x| -x)`
//!
//! To find a weakly increasing subsequence
//! (i.e. it can contain equal elements), pair each
//! element with its index.
//! TODO: maybe add a function for this?

use std::cmp::Ordering;

/// Returns a longest increasing list of indices such that
/// the `list[i]` values are also strictly increasing.
///
/// If there can be multiple such lists, returns one
/// of them. This function is deterministic though, so the
/// same input gives the same output list (as long as the
/// comparison is also deterministic, of course).
///
/// 🕒 `O(n logn)`.
///
/// # Example
/// ```
/// # use lis::lis;
/// assert_eq!(lis(&[0i32, 1, -2, 2]), vec![0, 1, 3]);
/// assert_eq!(lis(&[0i32, 0, 0]).len(), 1);
/// ```
pub fn lis<T: Ord>(list: &[T]) -> Vec<usize> {
    lis_by(list, T::cmp)
}

/// Returns a longest increasing list of indices such that
/// the `list[i]` values are also strictly increasing
/// in terms of `compare`.
///
/// If there can be multiple such lists, returns one
/// of them. This function is deterministic though, so the
/// same input gives the same output list (as long as the
/// comparison is also deterministic, of course).
///
/// 🕒 `O(n logn)`.
///
/// # Example
/// ```
/// # use lis::lis_by;
/// assert_eq!(
///     lis_by(&[0i32, -1, 2, -2], |x, y| x.cmp(y).reverse()),
///     vec![0, 1, 3]);
/// assert_eq!(
///     lis_by(&[0i32, 0, 0], |x, y| x.cmp(y)).len(),
///     1
/// );
/// ```
pub fn lis_by<T, F>(list: &[T], compare: F) -> Vec<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    if list.is_empty() {
        return vec![];
    }

    let n = list.len();
    let mut prev = Vec::<usize>::with_capacity(n);
    let mut optimal_ends = vec![];
    for (i, x) in list.iter().enumerate() {
        let prev_i = optimal_ends.partition_point(|j| compare(x, &list[*j]).is_gt());
        if prev_i == 0 {
            prev.push(0);
        } else {
            prev.push(optimal_ends[prev_i - 1]);
        }
        if prev_i == optimal_ends.len() {
            optimal_ends.push(i);
        } else {
            optimal_ends[prev_i] = i;
        }
    }

    let mut lis = optimal_ends;
    let mut i = *lis.last().unwrap();
    for x in lis.iter_mut().rev() {
        *x = i;
        i = prev[i];
    }
    lis
}

/// Returns a longest increasing list of indices such that
/// the `f(list[i])` values are also strictly increasing.
///
/// If there can be multiple such lists, returns one
/// of them. This function is deterministic though, so the
/// same input gives the same output list (as long as the
/// comparison is also deterministic, of course).
///
/// 🕒 `O(n logn)`.
///
/// # Example
/// ```
/// # use lis::lis_by_key;
/// assert_eq!(
///     lis_by_key(&[0i32, -1, 2, -2], |x| -x),
///     vec![0, 1, 3]);
/// assert_eq!(
///     lis_by_key(&[0i32, 0, 0], |x| *x).len(),
///     1
/// );
/// ```
pub fn lis_by_key<T, F, K>(list: &[T], f: F) -> Vec<usize>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    let keys = list.iter().map(f).collect::<Vec<_>>();
    lis(&keys)
}

#[cfg(test)]
mod test {
    use crate::lis;

    #[test]
    fn test_lis() {
        fn naive<T: Ord>(list: &[T]) -> usize {
            let mut dp = vec![];
            for (i, x) in list.iter().enumerate() {
                let mut ans = 1;
                for (j, y) in list[..i].iter().enumerate() {
                    if x > y {
                        ans = ans.max(dp[j] + 1);
                    }
                }
                dp.push(ans);
            }
            dp.into_iter().max().unwrap_or(0)
        }

        fn check(list: &[i64]) {
            println!("{list:?}");
            let lis_mine = lis(list);
            let ans_naive = naive(list);

            // length is correct
            assert_eq!(lis_mine.len(), ans_naive);
            // indices are increasing and valid,
            // list is increasing
            for i in &lis_mine {
                assert!(*i < list.len())
            }
            for w in lis_mine.windows(2) {
                assert!(w[0] < w[1]);
                assert!(list[w[0]] < list[w[1]]);
            }
        }

        check(&[]);
        check(&[1]);
        for a in -3..=3 {
            check(&[a]);
            for b in -3..=3 {
                check(&[a, b]);
                for c in -3..=3 {
                    check(&[a, b, c]);
                    for d in -3..=3 {
                        check(&[a, b, c, d]);
                    }
                }
            }
        }
        check(&[1, 2, 3, 4, 5, 6, 7]);
        check(&[3, 4, 5, 6, 2, 3, 1, 7, 4, 3, 5, 6, 7]);
    }
}
