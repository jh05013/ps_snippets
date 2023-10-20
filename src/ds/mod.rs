//! Data structures.

/// A static range sum data structure.
/// 
/// # Examples
/// ```
/// # use ps_snippets::ds::range_sum::RangeSum;
/// let rs = RangeSum::from_vec(vec![1, 3, 5, 7, 9]);
/// assert_eq!(rs.len(), 5);
/// assert_eq!(rs.pref_sum(2), 1+3+5);
/// assert_eq!(rs.sum(1, 3), 3+5+7);
/// assert_eq!(rs.whole(), 1+3+5+7+9);
/// let mut rs2 = rs;
/// rs2.push(11);
/// assert_eq!(rs2.sum(3, 5), 7+9+11);
/// ```
/// 
/// # Practice Problems
/// - [LC Static Range Sum](https://judge.yosupo.jp/problem/static_range_sum)
/// - [BOJ 16139 인간-컴퓨터 상호작용](https://www.acmicpc.net/problem/16139)
pub mod range_sum;

/// A merge-sort tree.
/// 
/// # Initialization
/// - `new(vals: Vec<T>)` constructs a merge-sort tree out of `vals`.
///   Takes $O(n \log n)$ where $n = |vals|$.
/// 
/// # Queries
/// - `len()` returns `n`.
/// - `count(l, r, val_l, val_r)` returns the number of elements in
///   `vals[l..=r] whose value lies in the range `[val_l..=val_r]`.
///   Takes $O(n \log^2 n)$.
/// 
/// # Caution
/// `count` panics if `0 <= l <= r < n` and `val_l <= val_r` are not satisfied.
/// 
/// # Examples
/// ```
/// # use ps_snippets::ds::merge_sort_tree::MergeSortTree;
/// let mst = MergeSortTree::new(vec![1, 3, 5, 2, 4, 6]);
/// assert_eq!(mst.count(1, 3, 3, 10), 2); // <3>, <5>, 2
/// ```
pub mod merge_sort_tree;

/// A lazy segment tree.
/// 
/// To use it, first define a zero-sized type and implement the
/// following `LazyMonoid` trait on it:
/// - `type V`: the type of the node value.
/// - `type Lazy`: the type of the lazy operation.
/// - `const ID: Self`: the identity element of the value operation.
/// - `const LAZY_ID: Lazy`: the identity element of the lazy operation.
/// - `op(lhs: Self, rhs: Self) -> Self`: the value operation.
/// - `op_lazy(cur: Lazy, up: Lazy) -> Lazy`: the lazy operation, where
///   `cur` is the current lazy of the node and `up` is the lazy which is
///   propagating from the parent node.
/// - `unlazy(v: Self, size: usize, lz: Lazy) -> Self`:
///   the updated value from `v` with lazy `lz` on a node of size `size`.
/// 
/// Both `V` and `Lazy` must implement [`Copy`].
/// 
/// Let $O(t_V)$ and $O(t_L)$ be the time complexity of `op` and `op_lazy`
/// respectively.
/// 
/// # Initialization
/// - `new(n)` constructs a lazy segtree with size `n` filled with `T::ID`.
///   Takes $O(n)$.
/// - `from_slice(vals: &[T::V]) constructs a lazy segtree out of `vals`.
///   Takes $O(|vals| t_L)$.
/// 
/// # Queries
/// - `update(l, r, lz: T::lazy)` updates all indices in `[l,r]` with `lz`.
///   Takes $O(t_L \log n)$.
/// - `query(l, r)` returns the result of the operation over all indices in
///   `[l,r]`. Takes $O((t_V + t_L) \log n)$.
/// 
/// # Caution
/// - If `LazyMonoid` does not satisfy the condition of a lazy segtree,
///   the behavior is unspecified and meaningless.
/// - `update` and `query` panics if `0 <= l <= r < n` is not satisfied.
/// 
/// # Examples
/// ```
/// # use ps_snippets::ds::segtree_lazy::*;
/// struct St;
/// 
/// impl LazyMonoid for St {
///     type V = i64;
///     type Lazy = i64;
///     const ID: Self::V = 0;
///     const LAZY_ID: Self::Lazy = 0;
///     fn op(lhs: Self::V, rhs: Self::V) -> Self::V {
///         lhs + rhs
///     }
///     fn op_lazy(cur: Self::Lazy, up: Self::Lazy) -> Self::Lazy {
///         cur + up
///     }
///     fn unlazy(v: Self::V, size: usize, lz: Self::Lazy) -> Self::V {
///         v + (size as i64) * lz
///     }
/// }
/// 
/// let mut st = SegtreeLazy::<St>::from_slice(&[1,2,3,4,5]);
/// assert_eq!(st.query(1, 3), 2+3+4);
/// st.update(0, 2, 10); // [11, 12, 13, 4, 5]
/// assert_eq!(st.query(1, 3), 12+13+4);
/// ```
/// 
/// # Practice Problems
/// - [BOJ 10999 구간 합 구하기 2](https://www.acmicpc.net/problem/10999)
/// - [LC Range Affine Range Sum](https://judge.yosupo.jp/problem/range_affine_range_sum) = [BOJ 13925 수열과 쿼리 13](https://www.acmicpc.net/problem/13925)
pub mod segtree_lazy;
