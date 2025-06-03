//! Explanation

use std::{
    fmt::{Debug, Display},
    io::{self, BufWriter, Read, Stdout, Write},
    str::FromStr as F,
};

/// Easy reader/writer utility, but with shorter code
/// and less functionalities.
/// 
/// This reads the entire stdin at initialization, and
/// is slightly (1.1x?) slower than [`io`].
///
/// # Example
/// The following code solves [LC Many A+B](https://judge.yosupo.jp/problem/many_aplusb):
///
/// ```no_run
/// let mut oj = io_short::stdin();
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
pub struct OJ {
    inp: Vec<String>,
    out: BufWriter<Stdout>,
}

macro_rules! a {
    ($ty:ident) => {
        impl OJ {
            pub fn $ty(&mut self) -> $ty {
                self.parse()
            }
        }
    };
}
a!(i32);
a!(u32);
a!(i64);
a!(u64);
a!(usize);
a!(f64);

impl OJ {
    pub fn word(&mut self) -> String {
        self.inp.pop().expect("EOF")
    }

    /// Reads and returns a `T`.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    pub fn parse<T: F>(&mut self) -> T
    where
        <T as F>::Err: Debug,
    {
        self.word().parse().unwrap()
    }

	/// Reads `n` values into a [`Vec`], separated by whitespace.
    ///
    /// ⚠️ Panics on EOF or failure to parse.
    pub fn vec<T: F>(&mut self, n: usize) -> Vec<T>
    where
        <T as F>::Err: Debug,
    {
        (0..n).map(|_| self.parse()).collect()
    }

    /// Writes `val` in [`Display`] format.
    pub fn write<T: Display>(&mut self, val: T) -> &mut Self {
        write!(self.out, "{val}").unwrap();
        self
    }

    /// Writes `val` in [`Debug`] format.
    pub fn debug<T: Debug>(&mut self, val: T) -> &mut Self {
        write!(self.out, "{val:?}").unwrap();
        self
    }

    /// Writes a blank character.
    pub fn sp(&mut self) -> &mut Self {
        self.write(' ')
    }

    /// Writes a newline character.
    pub fn ln(&mut self) -> &mut Self {
        self.write('\n')
    }
}

pub fn stdin() -> OJ {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    OJ {
        inp: input
            .split_whitespace()
            .rev()
            .map(|s| s.to_owned())
            .collect(),
        out: BufWriter::with_capacity(1 << 18, io::stdout()),
    }
}
