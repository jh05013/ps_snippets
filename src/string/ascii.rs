pub mod ascii_mod {
	use std::ops::{Deref, DerefMut, Index, IndexMut};
	#[derive(Clone, Debug, Default, PartialOrd, Ord, PartialEq, Eq)]
	pub struct Ascii(pub String);

	impl Deref for Ascii { type Target = String;
		fn deref(&self) -> &Self::Target { &self.0 }
	}
	impl DerefMut for Ascii {
		fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
	}

	impl std::str::FromStr for Ascii { type Err = ();
		fn from_str(s: &str) -> Result<Self, Self::Err> {
			Ok(Ascii(s.to_string()))
		}
	}
	impl std::fmt::Display for Ascii {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.0)
		}
	}

	impl Index<usize> for Ascii { type Output = u8;
		fn index(&self, index: usize) -> &Self::Output {
			&self.as_bytes()[index]
		}
	}
	impl IndexMut<usize> for Ascii {
		fn index_mut(&mut self, index: usize) -> &mut Self::Output {
			// SAFETY: it's not. DON'T USE THIS OUTSIDE OF OJ'S!
			unsafe { &mut self.as_bytes_mut()[index] }
		}
	}
} pub use ascii_mod::Ascii;
