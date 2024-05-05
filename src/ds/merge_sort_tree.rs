pub mod merge_sort_tree_mod {
	/// The merge-sort tree.
	#[derive(Clone, Debug, Default)]
	pub struct MergeSortTree<T: Clone + PartialOrd> {
		n: usize,
		tree: Vec<Vec<T>>,
	}

	impl<T: Clone + PartialOrd> MergeSortTree<T> {
		/// Constructs a new merge-sort tree out of `vals`.
		pub fn new(vals: Vec<T>) -> Self {
			let n = vals.len();
			let mut tree = vec![vec![]; 2*n];
			vals.into_iter().enumerate().for_each(|(i, v)|
				tree[i+n] = vec![v]
			);
			for i in (1..n).rev() {
				let mut v = tree[2*i].clone();
				v.append(&mut tree[2*i+1].clone());
				v.sort_by(|a,b| a.partial_cmp(b).unwrap());
				tree[i] = v;
			}
			Self { n, tree }
		}
		/// Returns the size of `vals`.
		#[allow(clippy::len_without_is_empty)]
		pub fn len(&self) -> usize { self.n }

		/// Returns the number of elements in `vals[l..=r]` whose value lies in `[val_l..=val_r]`.
		pub fn count(&self, l: usize, r: usize, val_l: &T, val_r: &T) -> usize {
			assert!(l <= r && r < self.n, "Bad query range [{}, {}]", l, r);
			assert!(val_l <= val_r, "Bad query-value range");
			let mut ans = 0;
			let (mut l, mut r) = (l + self.n, r + self.n+1);
			while l < r {
				if l&1 == 1 {
					ans += Self::count_one(&self.tree[l], val_l, val_r);
					l += 1;
				}
				if r&1 == 1 {
					r -= 1;
					ans += Self::count_one(&self.tree[r], val_l, val_r);
				}
				l >>= 1; r >>= 1;
			}
			ans
		}

		fn count_one(arr: &[T], val_l: &T, val_r: &T) -> usize {
			let i = arr.partition_point(|x| x < val_l);
			arr[i..].partition_point(|x| x <= val_r)
		}
	}

	impl<T: Clone + PartialOrd> std::ops::Index<usize> for MergeSortTree<T> {
		type Output = T;
		fn index(&self, index: usize) -> &Self::Output {
			&self.tree[self.n + index][0]
		}
	}
}
pub use merge_sort_tree_mod::MergeSortTree;
