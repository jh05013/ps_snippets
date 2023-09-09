// Input/output wrapper
use std::io::*;
pub struct OJ {
	buffer: std::str::SplitWhitespace<'static>,
	pub out: BufWriter<Stdout>
}
impl OJ {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		let mut inp = String::new();
		stdin().read_to_string(&mut inp).unwrap();
		let input = Box::leak(inp.into_boxed_str()).split_whitespace();
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
	pub fn write<T: std::fmt::Display>(&mut self, v: T, end: &str) {
		write!(self.out, "{}{}", v, end).unwrap();
	}
	pub fn debug<T: std::fmt::Debug>(&mut self, v: T, end: &str) {
		write!(self.out, "{:?}{}", v, end).unwrap();
	}
	pub fn quit<T: std::fmt::Display>(&mut self, v: T) {
		self.write(v,""); self.out.flush().unwrap(); std::process::exit(0);
	}
}

// str
pub static EM: &str = "";
pub static SP: &str = " ";
pub static LN: &str = "\n";
