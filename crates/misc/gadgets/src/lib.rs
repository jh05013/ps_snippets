//! Tiny utilities that do not deserve their own crates.
//!
//! All functions here are private.
//! Instead of importing this module,
//! open the source code and copy-paste the functions you need.
//!
//! # Utilities
//! ## Sequences
//! - `is_subsequence`: checks a subsequence relation.

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
