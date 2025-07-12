//! Tiny utilities that do not deserve their own crates.
//!
//! All functions here are private.
//! Instead of importing this module,
//! open the source code and copy-paste the functions you need.

////////////////////////////////////////// MATH

#[allow(dead_code)]
/// Returns the number of integers `i` in `l..=r`
/// such that `x == i % md`.
fn count_mod_in_range(l: u64, r: u64, x: u64, md: u64) -> u64 {
    assert_ne!(md, 0);
    if x >= md || l > r {
        return 0;
    }
    let mut ans = r / md + (r % md >= x) as u64;
    if l > 0 {
        ans -= (l - 1) / md + ((l - 1) % md >= x) as u64;
    }
    ans
}

////////////////////////////////////////// DIGITS

#[allow(dead_code)]
/// Returns the digits of `n`, from right to left.
fn digits_rtol(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |i| Some(*i / 10))
        .take_while(|i| *i > 0)
        .map(|i| i % 10)
}

////////////////////////////////////////// SEQUENCE

#[allow(dead_code)]
/// Returns whether `small` is a subsequence of `large`.
fn is_subsequence<T: PartialEq>(
    small: impl Iterator<Item = T>,
    mut large: impl Iterator<Item = T>,
) -> bool {
    for x in small {
        if !large.any(|y| x == y) {
            return false;
        }
    }
    true
}
