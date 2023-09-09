#[allow(unused_imports)] use std::collections::*;
#[allow(unused_imports)] use std::{fmt,cmp,ops};

pub struct BTreeMultiset<T: Ord> {
	pub inner: BTreeMap<T, usize>
}

impl<T: Ord> BTreeMultiset<T> {
	pub fn new() -> Self { Self { inner: BTreeMap::new() } }
	pub fn first(&self) -> Option<&T> { self.inner.first_key_value().and_then(|(k,_)| Some(k)) }
	pub fn last(&self) -> Option<&T> { self.inner.last_key_value().and_then(|(k,_)| Some(k)) }

	pub fn insert(&mut self, value: T) {
		*self.inner.entry(value).or_insert(0) += 1;
	}
	pub fn remove(&mut self, value: T) -> bool {
		use std::collections::btree_map::Entry::*;
		let Occupied(mut ent) = self.inner.entry(value) else { return false; };
		*ent.get_mut() -= 1;
		if *ent.get() == 0 { ent.remove(); }
		true
	}
}

fn main() {
	let mut oj = OJ::new();
	let n: usize = oj.read();
	let q: usize = oj.read();
	let mut s = BTreeMultiset::<i32>::new();
	for _ in 0..n { s.insert(oj.read()); }

	for _ in 0..q {
		let qty: usize = oj.read();
		if qty == 0 { s.insert(oj.read()); }
		else if qty == 1 {
			let x = s.first().unwrap();
			oj.write(x, LN);
			s.remove(*x);
		}
		else {
			let x = s.last().unwrap();
			oj.write(x, LN);
			s.remove(*x);
		}
	}
}

////////////// LIB ////////////////////////

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
