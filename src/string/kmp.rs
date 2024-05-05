pub mod kmp_mod {
	pub fn partial_match<T: PartialEq>(word: &[T]) -> Vec<usize> {
		let mut fail = vec![0; word.len()+1];
		if word.len() == 1 { return fail; }
		let (mut pos, mut cnd) = (2, 0);
		while pos < word.len()+1 {
			if word[pos-1] == word[cnd] {
				fail[pos] = cnd+1; cnd += 1; pos += 1;
			}
			else if cnd > 0 { cnd = fail[cnd]; }
			else { fail[pos] = 0; pos += 1; }
		}
		fail
	}

	pub fn all_matches<T: PartialEq>(
		haystack: &[T], needle: &[T],
		partial: Option<Vec<usize>>
	) -> Vec<usize> {
		let partial = partial.unwrap_or_else(
			|| partial_match(needle)
		);
		let mut ans = vec![];
		let (mut m, mut i) = (0, 0);
		while m+i < haystack.len() {
			if i < needle.len() && needle[i] == haystack[m+i] {
				if i+1 == needle.len() { ans.push(m); }
				i += 1;
			}
			else if i > 0 { (m, i) = (m+i-partial[i], partial[i]); }
			else { m += 1; }
		}
		ans
	}
} pub use kmp_mod::{partial_match, all_matches};
