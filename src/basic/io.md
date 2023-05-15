# IO

Competitive programming uses so much stdin/stdout IO that Rustaceans in Online Judges tend to make their own IO templates. Here is my approach focusing on simplicity.

## `OJ`

- 💬 An IO wrapper.
- ℹ️ Upon creation, it reads the entire stdin, so you have to provide EOF by yourself when testing from cmdline.
- ⚠️ **This must be only used for competitive programming/problem solving, and will cause problems in general applications!**
- ⚠️ Only one wrapper must be made throughout the whole program.

**Fields**

- `out: BufWriter<Stdout>`: stdout.

`fn new() -> Self`

- 🏗️ Creates a new OJ wrapper.
- 🕒 Proportional to the whole input size.

`fn read<T: FromStr>(&mut self) -> T`

- 💬 Reads and returns a value from stdin.
- ⚠️ Panics on EOF.
- ⚠️ Panics if the token read cannot be parsed into `T`.

`fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>`

- 💬 Reads and returns a vector of \\( n \\) values from stdin.
- ⚠️ Panics on EOF.
- ⚠️ Panics if any token read cannot be parsed into `T`.

`fn write<T: Display>(&mut self, v: T, end: &str)`

- 💬 Writes `v` and then `end` to stdout.
- ℹ️ `end` can be any string, but the following shorthands are available:
  - `EM` is `""`.
  - `SP` is `" "`.
  - `LN` is `"\n"`.
- ℹ️ If you need more sophisticated formats, use `write!(oj.out, fmt, args)`.

`fn debug<T: Debug>(&mut self, v: T, end: &str)`

- 💬 Writes `v` as a debugging format and then `end` to stdout.

`fn flush(&mut self)`

- 💬 Flushes stdout.

`fn quit<T: Display>(&mut self, v: T)`

- 💬 Writes `v` to stdout and terminates the program.

`fn at(s: &String, i: usize) -> char`

- 💬 Returns `s[i]`.
- ℹ️ Not really IO, but it's used so much that I included it here.

## Example

The following code solves [BOJ 15552](https://www.acmicpc.net/problem/15552) (2M reads, 1M writes) in 116 ms:

```rust,noplayground
fn main() {
    let mut oj = OJ::new();
    for _ in 0..oj.read() {
        let a: i32 = oj.read();
        let b: i32 = oj.read();
        oj.write(a+b, LN);
    }
}
```

The following code solves [BOJ 27866](https://www.acmicpc.net/problem/27866):
```rust,noplayground
fn main() {
    let mut oj = OJ::new();
    let s: String = oj.read();
    let i: usize = oj.read::<usize>() - 1;
    oj.write(at(&s, i), EM);
}
```

## Code
```rust,noplayground
#[allow(dead_code)]
mod oj_io {
    use std::io::*;
    use std::fmt::{Debug, Display};

    pub struct OJ {
        buffer: std::str::SplitWhitespace<'static>,
        pub out: BufWriter<Stdout>
    }
    impl OJ {
        pub fn new() -> Self {
            let input = Box::leak(read_to_string(stdin()).unwrap().into_boxed_str()).split_whitespace();
            OJ { buffer: input, out: BufWriter::new(stdout()) }
        }

        // INPUT
        pub fn read<T: std::str::FromStr>(&mut self) -> T {
            self.buffer.next().unwrap().parse().ok().expect("Failed parse")
        }
        pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.read()).collect()
        }

        // OUTPUT
        pub fn write<T: Display>(&mut self, v: T, end: &str) { write!(self.out, "{}{}", v, end).unwrap(); }
        pub fn debug<T: Debug>(&mut self, v: T, end: &str) { write!(self.out, "{:?}{}", v, end).unwrap(); }
        pub fn flush(&mut self) { self.out.flush().unwrap(); }
        pub fn quit<T: Display>(&mut self, v: T) { self.write(v,""); self.flush(); std::process::exit(0); }
    }

    // str
    pub static EM: &str = "";
    pub static SP: &str = " ";
    pub static LN: &str = "\n";
    pub fn at(s: &String, i: usize) -> char { s.as_bytes()[i] as char }
} use oj_io::*;
```