//! IO.

use std::{io::*, str::*, fmt::*};

/// IO wrapper for online judge environments.
/// 
/// # Example
/// The following code solves [BOJ 15552](https://www.acmicpc.net/problem/15552)
/// and [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb):
/// ```no_run
/// use ps_snippets::io::*;
/// 
/// let mut oj = OJ::new();
/// for _ in 0..oj.read() {
///     let a: i64 = oj.read();
///     let b: i64 = oj.read();
///     oj.write(a+b, LN);
/// }
/// ```
/// 
/// # Practice Problems
/// - [BOJ 15552 빠른 A+B](https://www.acmicpc.net/problem/15552) `read`, `write`
/// - [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb)
/// - [BOJ 10951 A+B - 4](https://www.acmicpc.net/problem/10951) `try_read`
/// - [BOJ 2750 수 정렬하기](https://www.acmicpc.net/problem/2750) `read_vec`
pub struct OJ {
	buffer: std::str::SplitWhitespace<'static>,
	out: BufWriter<Stdout>
}

impl OJ {
	/// Creates an IO wrapper for stdin.
	/// 
	/// ⚠️ Do NOT make more than one instance of this,
	/// or try to directly read from stdin after creating this.
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut inp = String::new();
		stdin().read_to_string(&mut inp).unwrap();
		let input = Box::leak(inp.into_boxed_str()).split_whitespace();
		OJ { buffer: input, out: BufWriter::new(stdout()) }
	}

	/// Tries to read a value of type `T` from stdin.
	/// 
	/// On failure (`EOF` or `Failed parse`), returns [`Err`].
	pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, &str> {
		self.buffer.next().ok_or("EOF")?.parse().or(Err("Failed parse"))
	}

	/// Reads a value of type `T` from stdin, panicking on failure.
	pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }

	/// Reads `n` values of type `T` from stdin into [`Vec`],
	/// panicking on failure.
	pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
		(0..n).map(|_| self.read()).collect()
	}

	/// Prints `v`, followed by `end`.
	/// 
	/// If you need a more sophisticated output, try `write(format!(...), EM)`.
	/// 
	/// `end` can be any string, but the following shorthands are available:
	/// - [`EM`] is an empty string.
	/// - [`SP`] is a space.
	/// - [`LN`] is a newline.
	pub fn write<T: Display>(&mut self, v: T, end: &str) {
		write!(self.out, "{}{}", v, end).unwrap();
	}

	/// Prints `v` in the debugging format, followed by `end`.
	/// 
	/// `end` can be any string, but the following shorthands are available:
	/// - [`EM`] is an empty string.
	/// - [`SP`] is a space.
	/// - [`LN`] is a newline.
	pub fn debug<T: Debug>(&mut self, v: T, end: &str) {
		write!(self.out, "{:?}{}", v, end).unwrap();
	}

	/// Prints `v` and exits the program.
	pub fn quit<T: Display>(&mut self, v: T) {
		self.write(v,""); self.out.flush().unwrap(); std::process::exit(0);
	}
}

/// Empty string (`""`).
pub static EM: &str = "";
/// Space (`" "`).
pub static SP: &str = " ";
/// Newline (`\n`).
pub static LN: &str = "\n";
