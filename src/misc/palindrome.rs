pub mod palindrome_mod {
	pub struct AllPalindromes {
		n: u32, cut: u32, odd: bool
	}

	impl Iterator for AllPalindromes {
		type Item = u64;

		fn next(&mut self) -> Option<Self::Item> {
			let s = format!("{}", self.n);
			let rs = s.chars().rev().collect::<String>();
			let srs = if self.odd { s + &rs[1..] } else { s + &rs };
			self.n += 1;
			if self.n == self.cut {
				if self.odd { self.n /= 10; self.odd = false; }
				else { self.cut *= 10; self.odd = true; }
			}
			Some(srs.parse::<u64>().unwrap())
		}
	}

	pub fn all_palindromes() -> AllPalindromes {
		AllPalindromes { n: 0, cut: 10, odd: true }
	}
} use palindrome_mod::{all_palindromes};
