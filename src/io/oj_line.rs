pub mod oj_line {
	use std::{io::*, str::*, fmt::*};

	pub struct OJ<'a> {
		lock: StdinLock<'a>,
		buffer: Vec<String>,
		out: BufWriter<Stdout>
	}
	
	impl OJ<'_> {
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			Self { lock: stdin().lock(), buffer: vec![], out: BufWriter::new(stdout()) }
		}

		pub fn try_load_line(&mut self) -> bool {
			let mut s = "".to_string();
			if self.lock.read_line(&mut s).unwrap() == 0 { return false; }
			self.buffer = s.split_whitespace().rev()
				.map(|s| s.to_string()).collect::<Vec<_>>();
			true
		}
	
		pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, &str> {
			let Some(tok) = self.buffer.pop() else { return Err("No more token on line"); };
			tok.parse().or(Err("Failed parse"))
		}
		pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }
		pub fn read_line<T:FromStr>(&mut self) -> Vec<T>
		where <T as std::str::FromStr>::Err: std::fmt::Debug {
			if self.buffer.is_empty() { assert!(self.try_load_line()); }
			std::mem::take(&mut self.buffer).into_iter().rev()
				.map(|s| s.parse().unwrap()).collect()
		}

		pub fn write<T: Display>(&mut self, v: T) -> &mut Self {
			write!(self.out, "{}", v).unwrap(); self
		}
		pub fn debug<T: Debug>(&mut self, v: T) -> &mut Self {
			write!(self.out, "{:?}", v).unwrap(); self
		}
		pub fn sp(&mut self) -> &mut Self { self.write(' ') }
		pub fn ln(&mut self) -> &mut Self { self.write('\n') }
		pub fn quit<T: Display>(&mut self, v: T) {
			self.write(v); self.out.flush().unwrap(); std::process::exit(0);
		}
	}
}
pub use oj_line::*;
