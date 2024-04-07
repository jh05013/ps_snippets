pub mod suffix_array_mod {
	// https://bamgoesn.github.io/rust-ps-md/strings/salcp.html

	pub fn suffix_array<T: Ord>(s: &[T]) -> Vec<usize> {
		if s.is_empty() { return vec![]; }
		else if s.len() == 1 { return vec![0]; }

		let n = s.len();
		let mut r: Vec<usize> = vec![0; n*2];
		let map: std::collections::BTreeMap<_, _> = {
			let mut sorted: Vec<_> = s.iter().collect();
			sorted.sort_unstable();
			sorted.into_iter().enumerate()
				.map(|x| (x.1, x.0+1)).collect()
		};
		for i in 0..n { r[i] = map[&s[i]]; }

		let m = n.max(map.len()) + 1;
		let mut sa: Vec<usize> = (0..n).collect();
		let mut nr: Vec<usize> = vec![0; n*2];
		let mut cnt: Vec<usize> = vec![0; m];
		let mut idx: Vec<usize> = vec![0; n];

		for d in (0..).map(|x| 1 << x).take_while(|&d| d < n) {
			macro_rules! key { ($i:expr) => {
				if $i + d >= n { (r[$i], 0) } else { (r[$i], r[$i + d]) }
			}; }
			(0..m).for_each(|i| cnt[i] = 0);
			(0..n).for_each(|i| cnt[r[i + d]] += 1);
			(1..m).for_each(|i| cnt[i] += cnt[i - 1]);
			for i in (0..n).rev() {
				cnt[r[i + d]] -= 1;
				idx[cnt[r[i + d]]] = i;
			}
			(0..m).for_each(|i| cnt[i] = 0);
			(0..n).for_each(|i| cnt[r[i]] += 1);
			(1..m).for_each(|i| cnt[i] += cnt[i - 1]);
			for i in (0..n).rev() {
				cnt[r[idx[i]]] -= 1;
				sa[cnt[r[idx[i]]]] = idx[i];
			}
			nr[sa[0]] = 1;
			for i in 1..n {
				nr[sa[i]] = nr[sa[i-1]] + usize::from(key!(sa[i-1]) < key!(sa[i]));
			}
			std::mem::swap(&mut r, &mut nr);
			if r[sa[n-1]] == n { break; }
		}
		sa
	}

	pub fn sa_lcp<T: Ord>(arr: &[T]) -> (Vec<usize>, Vec<usize>) {
		let n = arr.len();
		let sa = suffix_array(arr);
		let mut lcp: Vec<usize> = vec![0; n];
		let mut isa: Vec<usize> = vec![0; n];
		for i in 0..n { isa[sa[i]] = i; }
		let mut k = 0;
		for i in 0..n {
			if isa[i] != 0 {
				let j = sa[isa[i]-1];
				while i+k < n && j+k < n && arr[i+k] == arr[j+k] { k += 1; }
				lcp[isa[i]] = if k == 0 { 0 } else { k -= 1; k + 1 };
			}
		}
		(sa, lcp)
	}

	/// If `needle` is a subslice of `s`, returns `Some(l, r)` where
	/// `sa[i]` are the starting indices of `needle` in `s`.
	/// Otherwise, returns `None`.
	pub fn binsearch_in_sa<T: Ord>(s: &[T], needle: &[T], sa: &[usize])
	-> Option<(usize, usize)> {
		let l = sa.partition_point(|&si| &s[si..] < needle);
		if !s[sa[l]..].starts_with(needle) { return None; }
		let r = sa[l..].partition_point(|&si|
			s[si..].starts_with(needle)
		) + l;
		Some((l, r-1))
	}
}
pub use suffix_array_mod::{suffix_array, sa_lcp, binsearch_in_sa};
