//! Easy I/O utilities.

use std::{
    fmt::{Debug, Display},
    io::{self, BufRead, BufWriter, Lines, StdinLock, Stdout, Write},
    iter::Peekable,
    process::exit,
    str::FromStr,
};

/// Easy reader/writer utility.
///
/// # Example
/// The following code solves [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb):
///
/// ```no_run
/// let mut oj = io::stdin();
/// for _ in 0..oj.usize() {
///     let a = oj.u64();
///     let b = oj.u64();
///     oj.write(a + b).ln();
/// }
/// ```
///
/// ## Problems
/// - Various "Multiple A+B" problems
///   - [BOJ 15552 빠른 A+B](https://www.acmicpc.net/problem/15552)
///   - [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb)
///   - [DMOJ A Plus B](https://dmoj.ca/problem/aplusb)
pub struct OJ<R: BufRead, W: Write> {
    lines: Peekable<Lines<R>>,
    out: W,
    tokens: Vec<String>,
}

impl<R: BufRead, W: Write> OJ<R, W> {
    /// Builds a reader/writer.
    #[inline]
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            lines: reader.lines().peekable(),
            out: writer,
            tokens: vec![],
        }
    }

    /// Returns `true` iff there are no more lines to read.
    ///
    /// ⚠️ This may work unintuitively if the file does not
    /// end with 0 or 1 newlines. I want to change the behavior
    /// someday, but for now, remember that it checks whether
    /// there are no more **lines** to read, not words.
    #[inline]
    pub fn is_eof(&mut self) -> bool {
        self.tokens.is_empty() && self.lines.peek().is_none()
    }

    /// Reads and returns a line.
    ///
    /// ⚠️ Panics on EOF.
    ///
    /// ⚠️ Panics if the current line hasn't finished reading.
    /// For example, if the input is `1 2\n3 4`, you cannot
    /// call `i32()` once and then `line()`.
    #[inline]
    pub fn line(&mut self) -> String {
        assert!(
            self.tokens.is_empty(),
            "please finish the current line before reading the next"
        );
        self.lines.next().expect("EOF").unwrap()
    }

    /// Reads and returns a "word".
    /// Each word is a string separated by spaces.
    ///
    /// ⚠️ Panics on EOF.
    #[inline]
    pub fn word(&mut self) -> String {
        while self.tokens.is_empty() {
            self.tokens = self
                .line()
                .split_whitespace()
                .rev()
                .map(ToString::to_string)
                .collect();
        }
        self.tokens.pop().unwrap()
    }

    /// Reads and returns the list of [`char`]s in `word()`.
    ///
    /// ⚠️ Panics on EOF.
    #[inline]
    pub fn chars(&mut self) -> Vec<char> {
        self.word().chars().collect()
    }

    fn parse<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        self.word().parse().unwrap()
    }

    /// Reads `n` values into a [`Vec`], separated by whitespace.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
    where
        <T as FromStr>::Err: Debug,
    {
        (0..n).map(|_| self.parse()).collect()
    }

    /// Reads and returns an `i32`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn i32(&mut self) -> i32 {
        self.parse()
    }

    /// Reads and returns an `i64`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn i64(&mut self) -> i64 {
        self.parse()
    }

    /// Reads and returns an `usize`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn usize(&mut self) -> usize {
        self.parse()
    }

    /// Reads and returns an `u32`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn u32(&mut self) -> u32 {
        self.parse()
    }

    /// Reads and returns an `u64`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn u64(&mut self) -> u64 {
        self.parse()
    }

    /// Reads and returns an `f64`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    #[inline]
    pub fn f64(&mut self) -> f64 {
        self.parse()
    }

    /// Writes `val` in [`Display`] format.
    #[inline]
    pub fn write<T: Display>(&mut self, val: T) -> &mut Self {
        write!(self.out, "{val}").unwrap();
        self
    }

    /// Writes `val` in [`Debug`] format.
    #[inline]
    pub fn debug<T: Debug>(&mut self, val: T) -> &mut Self {
        write!(self.out, "{val:?}").unwrap();
        self
    }

    /// Writes a blank character.
    #[inline]
    pub fn sp(&mut self) -> &mut Self {
        self.write(' ')
    }

    /// Writes a newline character.
    #[inline]
    pub fn ln(&mut self) -> &mut Self {
        self.write('\n')
    }

    /// Flushes the output.
    #[inline]
    pub fn flush(&mut self) -> &mut Self {
        self.out.flush().unwrap();
        self
    }

    /// Writes `val` in [`Display`] format and quits.
    #[inline]
    pub fn quit<T: Display>(&mut self, val: T) -> ! {
        self.write(val);
        self.out.flush().unwrap();
        exit(0)
    }
}

/// Builds a fast reader/writer out of stdin and stdout.
#[must_use]
#[inline]
pub fn stdin() -> OJ<StdinLock<'static>, BufWriter<Stdout>> {
    let lock = io::stdin().lock();
    OJ::new(lock, BufWriter::with_capacity(1 << 18, io::stdout()))
}

#[cfg(test)]
mod test {
    use crate::OJ;

    #[test]
    fn test_read() {
        let input = "fox  1 0.5\nas  df\n".as_bytes();
        let mut oj = OJ::new(input, vec![]);
        assert_eq!(oj.word(), "fox");
        assert_eq!(oj.i32(), 1);
        assert_eq!(oj.f64(), 0.5);
        assert_eq!(oj.line(), "as  df");
        assert!(oj.is_eof());
    }

    #[test]
    fn test_read_vec() {
        let input = "1 2\n3 4   the string".as_bytes();
        let mut oj = OJ::new(input, vec![]);
        assert_eq!(oj.vec::<u32>(4), vec![1, 2, 3, 4]);
        assert_eq!(
            oj.vec::<String>(2),
            vec!["the".to_string(), "string".to_string()]
        );
    }

    #[test]
    #[should_panic]
    fn test_read_line_too_early() {
        let input = "as df\nas df".as_bytes();
        let mut oj = OJ::new(input, vec![]);
        oj.word();
        oj.line(); // not allowed; should finish reading the current line
    }
}
