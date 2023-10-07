pub mod oj_interact {
	use std::{io::*, str::*};

	pub struct OJInteract {
		buffer: Vec<String>,
	}
	
	impl OJInteract {
		#[allow(clippy::new_without_default)]
		pub fn new() -> Self {
			Self { buffer: vec![] }
		}
	
		pub fn try_read<T: FromStr>(&mut self) -> std::result::Result<T, &str> {
			while self.buffer.is_empty() {
				let mut s = "".to_string();
				stdin().read_line(&mut s).or(Err("EOF"))?;
				self.buffer = s.split_whitespace().rev()
					.map(|s| s.to_string()).collect::<Vec<_>>();
			}
			self.buffer.pop().unwrap().parse().or(Err("Failed parse"))
		}
		pub fn read<T: FromStr>(&mut self) -> T { self.try_read().unwrap() }
		pub fn read_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
			(0..n).map(|_| self.read()).collect()
		}

		pub fn quit() { std::process::exit(0); }
	}
} pub use oj_interact::*;
