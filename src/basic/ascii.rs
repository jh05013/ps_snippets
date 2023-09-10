use std::{ops::*, str::*};
pub struct Ascii(pub Vec<char>);

impl FromStr for Ascii {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if !s.is_ascii() { Err(()) }
		else { Ok(Self(s.chars().collect())) }
	}
}
impl Index<usize> for Ascii {
	type Output = char;
	fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
}
impl IndexMut<usize> for Ascii {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}
impl Ascii {
	pub fn join(&self) -> String { self.0.iter().collect::<String>() }
}
