mod ascii {
	use std::{ops::*, str::*};

	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
	pub struct Ascii(pub Vec<char>);
	
	// IO
	impl FromStr for Ascii {
		type Err = ();
		fn from_str(s: &str) -> Result<Self, Self::Err> {
			if !s.is_ascii() { Err(()) }
			else { Ok(Self(s.chars().collect())) }
		}
	}
	impl std::fmt::Display for Ascii {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			f.write_str(&self.s())
		}
	}
	
	// Str conversion
	impl Ascii {
		pub fn s(&self) -> String { self.0.iter().collect::<String>() }
	}
	impl From<String> for Ascii {
		fn from(value: String) -> Self {
			FromStr::from_str(&value).unwrap()
		}
	}
	
	// Indexing
	impl Index<usize> for Ascii {
		type Output = char;
		fn index(&self, index: usize) -> &Self::Output { &self.0[index] }
	}
	impl IndexMut<usize> for Ascii {
		fn index_mut(&mut self, index: usize) -> &mut Self::Output {
			&mut self.0[index]
		}
	}	
} pub use ascii::Ascii;
