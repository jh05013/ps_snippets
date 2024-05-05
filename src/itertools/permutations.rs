pub mod permutations_mod {
	pub fn make_next_permutation<T: PartialOrd>
	(arr: &mut [T]) -> bool {
		let n = arr.len();
		let Some(i) = (1..n).rev().find(|i| arr[i-1] < arr[*i])
			else { return false; };
		let j = (i..n).rev().find(|j| arr[i-1] < arr[*j]).unwrap();
		arr.swap(i-1, j);
		arr[i..].reverse();
		true
	}
} pub use permutations_mod::make_next_permutation;
