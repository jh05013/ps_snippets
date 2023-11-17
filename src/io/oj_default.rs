pub mod oj_default_mod {
	use std::{io::*, str::*, fmt::*};

	pub struct OJ {
		buffer: std::str::SplitWhitespace<'static>,
		out: BufWriter<Stdout>
	}
	
	impl OJ {
		/// Constructs a new OJ IO interface.
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			let mut inp = String::new();
			stdin().read_to_string(&mut inp).unwrap();
			let input = Box::leak(inp.into_boxed_str()).split_whitespace();
			Self { buffer: input, out: BufWriter::new(stdout()) }
		}
	
		/// Tries to read a type `T`.
		pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, &str> {
			self.buffer.next().ok_or("EOF")?.parse().or(Err("Failed parse"))
		}
		/// Reads a type `T`, panicking on failure.
		/// `i32`, `i64`, `u32`, `u64`, `f64`, `string` are its specializations.
		pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }
		pub fn i32(&mut self) -> i32 { self.read() }
		pub fn i64(&mut self) -> i64 { self.read() }
		pub fn u32(&mut self) -> u32 { self.read() }
		pub fn u64(&mut self) -> u64 { self.read() }
		pub fn f64(&mut self) -> f64 { self.read() }
		pub fn string(&mut self) -> String { self.read() }
		/// Reads `n` values of type `T` into a [`Vec`], panicking on failure.
		pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
			(0..n).map(|_| self.read()).collect()
		}

		/// Writes `v`.
		pub fn write<T: Display>(&mut self, v: T) -> &mut Self {
			write!(self.out, "{v}").unwrap(); self
		}
		/// Writes `v` in debug form.
		pub fn debug<T: Debug>(&mut self, v: T) -> &mut Self {
			write!(self.out, "{v:?}").unwrap(); self
		}
		/// Writes `' '`.
		pub fn sp(&mut self) -> &mut Self { self.write(' ') }
		/// Writes `'\n'`.
		pub fn ln(&mut self) -> &mut Self { self.write('\n') }
		/// Writes `v` and exits.
		pub fn quit<T: Display>(&mut self, v: T) -> ! {
			self.write(v); self.out.flush().unwrap(); std::process::exit(0)
		}
	}
}
pub use oj_default_mod::OJ;
