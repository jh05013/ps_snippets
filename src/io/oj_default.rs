pub mod oj_default_mod {
	use std::{fmt::{Display, Debug}, io::*, process::*, str::*};

	pub struct OJ {
		buffer: std::str::SplitWhitespace<'static>,
		out: BufWriter<Stdout>
	}
	
	impl OJ {
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			let mut inp = String::new();
			stdin().read_to_string(&mut inp).unwrap();
			let input = Box::leak(inp.into_boxed_str()).split_whitespace();
			Self { buffer: input, out: BufWriter::with_capacity(1<<18, stdout()) }
		}
	
		pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, String> {
			let s = self.buffer.next().ok_or("EOF")?;
			s.parse().or(Err(format!("Failed parse: {s}")))
		}
		pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }
		pub fn i32(&mut self) -> i32 { self.read() }
		pub fn i64(&mut self) -> i64 { self.read() }
		pub fn u32(&mut self) -> u32 { self.read() }
		pub fn u64(&mut self) -> u64 { self.read() }
		pub fn usize(&mut self) -> usize { self.read() }
		pub fn f64(&mut self) -> f64 { self.read() }
		pub fn string(&mut self) -> String { self.read() }

		pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T>
			{ (0..n).map(|_| self.read()).collect() }
		pub fn read_grid<T: FromStr>(&mut self, n: usize, m: usize) -> Vec<Vec<T>>
			{ (0..n).map(|_| self.read_vec(m)).collect() }

		pub fn write<T: Display>(&mut self, v: T) -> &mut Self
			{ write!(self.out, "{v}").unwrap(); self }
		pub fn writes<T: Display>(&mut self, vals: &[T]) -> &mut Self
			{ for v in vals { self.write(v).sp(); } self }
		pub fn debug<T: Debug>(&mut self, v: T) -> &mut Self
			{ write!(self.out, "{v:?}").unwrap(); self }
		pub fn sp(&mut self) -> &mut Self { self.write(' ') }
		pub fn ln(&mut self) -> &mut Self { self.write('\n') }
		pub fn quit<T: Display>(&mut self, v: T) -> !
			{ self.write(v); self.out.flush().unwrap(); exit(0) }
	}
}
pub use oj_default_mod::OJ;
