#[allow(unused_imports)] use std::{collections::*, cmp::*, ops::*};

fn main() {
	let mut oj = OJ::new();
	
}

///////////////////////////////////////

pub mod oj {
	use std::{io::*, str::*, fmt::*};

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
			OJ { buffer: input, out: BufWriter::new(stdout()) }
		}
	
		pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, &str> {
			self.buffer.next().ok_or("EOF")?.parse().or(Err("Failed parse"))
		}
		pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }
		pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
			(0..n).map(|_| self.read()).collect()
		}

		pub fn write<T: Display>(&mut self, v: T, end: &str) {
			write!(self.out, "{}{}", v, end).unwrap();
		}
		pub fn debug<T: Debug>(&mut self, v: T, end: &str) {
			write!(self.out, "{:?}{}", v, end).unwrap();
		}
		pub fn quit<T: Display>(&mut self, v: T) {
			self.write(v,""); self.out.flush().unwrap(); std::process::exit(0);
		}
	}
	
	pub static EM: &str = "";
	pub static SP: &str = " ";
	pub static LN: &str = "\n";
} use oj::*;
