//! Strings.

/// ASCII strings.
/// 
/// Supports printing, indexing, and derefs into [`String`].
/// Indexing gives [`u8`] instead of [`char`], but it's close enough.
/// 
/// Since it implements `FromStr`, you can directly `read`
/// it with any `ps_snippets::io` module.
/// 
/// # Examples
/// ```
/// # use ps_snippets::string::ascii::Ascii;
/// let mut a = Ascii("asdf".to_string());
/// assert_eq!(a[1], ('s' as u8));
/// a[2] = '@' as u8;
/// assert_eq!(a[2], ('@' as u8));
/// assert_eq!(a.len(), 4); // Deref method
/// // let s: Ascii = oj.read();
/// // oj.write(s);
/// ```
/// 
/// # Practice Problems
/// - [BOJ 27866 문자와 문자열](https://www.acmicpc.net/problem/27866)
pub mod ascii;

/// Suffix array.
/// 
/// Base code was taken from [bamgoeSN](https://bamgoesn.github.io/rust-ps-md/strings/salcp.html).
/// 
/// TODO: change to a struct.
pub mod suffix_array;

pub mod rabin_karp;
