//! Data structures.

/// A static range sum data structure.
/// 
/// TODO: support non-invertible prefix operations, e.g. max.
/// 
/// # Initialization
/// - `new()` constructs an empty range sum DS.
/// - `from_vec(vals: Vec<T>)` constructs a range sum DS out of `vals`.
/// 
/// # Queries
/// - `len()` returns `n`.
/// - `is_empty()` returns true iff `vals` is empty.
/// - `sum(i, j)` returns `vals[i] + ... + vals[j]`.
/// - `whole()` returns the sum of all `vals[_]`, or 0 if empty.
/// 
/// # Updates
/// - `push(v)` inserts `v` at the end of `vals`.
/// - `resize(n)` truncates or extends (with 0) `vals` to length `n`.
/// 
/// # Caution
/// `sum` panics if `0 <= l <= r < n` are not satisfied.
/// 
/// # Examples
/// ```
/// # use ps_snippets::ds::range_sum::RangeSum;
/// let rs = RangeSum::from_vec(vec![1, 3, 5, 7, 9]);
/// assert_eq!(rs.len(), 5);
/// assert_eq!(rs.sum(0, 2), 1+3+5);
/// assert_eq!(rs.sum(1, 3), 3+5+7);
/// assert_eq!(rs.whole(), 1+3+5+7+9);
/// let mut rs2 = rs;
/// rs2.push(11);
/// assert_eq!(rs2.sum(3, 5), 7+9+11);
/// rs2.resize(2);
/// assert_eq!(rs2.whole(), 1+3);
/// ```
/// 
/// # Practice Problems
/// - [LC Static Range Sum](https://judge.yosupo.jp/problem/static_range_sum)
/// - [BOJ 16139 인간-컴퓨터 상호작용](https://www.acmicpc.net/problem/16139)
pub mod range_sum;

/// A merge-sort tree.
/// 
/// `T` must implement [`Clone`] + [`PartialOrd`] + [`std::fmt::Display`] (for error messages).
/// 
/// # Initialization
/// - `new(vals: Vec<T>)` constructs a merge-sort tree out of `vals`.
///   Takes $O(n \log n)$ where $n = |vals|$.
/// 
/// # Queries
/// - `len()` returns `n`.
/// - `count(l, r, val_l, val_r)` returns the number of elements in
///   `vals[l..=r]` whose value lies in the range `[val_l..=val_r]`.
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
/// 
/// # Practice Problems
/// - [BOJ 13537 수열과 쿼리 1](https://www.acmicpc.net/problem/13537) 1e5 elements, 1e5 queries
/// - [BOJ 13544 수열과 쿼리 3](https://www.acmicpc.net/problem/13544) 1e5 elements, 1e5 queries
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
/// The following implementation supports range additon and range sum.
/// Note that `RangeFenwick` (TODO) is better for this task,
/// and this example is just for demonstration:
/// 
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
/// - [BOJ 10999 구간 합 구하기 2](https://www.acmicpc.net/problem/10999) 1e6 elements, 2e4 queries
/// - [LC Range Affine Range Sum](https://judge.yosupo.jp/problem/range_affine_range_sum) 5e5 elements, 5e5 queries
pub mod segtree_lazy;

/// A minimum-in-queue data structure, commonly called deque DP trick.
/// 
/// Notably, all operations take amortized constant time.
/// 
/// If you need a maximum-in-queue data structure, wrap it with [`std::cmp::Reverse`].
/// 
/// `T` must implement [`PartialOrd`].
/// 
/// # Initialization
/// - `new()` initializes an empty queue.
/// 
/// # Basic Methods
/// - `len()` returns the length of the queue.
/// - `is_empty()` returns `true` iff the queue is empty.
/// 
/// # Update
/// - `push(v)` pushes `v` into the queue.
/// - `pop()` pops from the queue and returns `true` if the queue is not empty; otherwise returns `false`.
/// 
/// Do not be confused with the internal data structure used to implement this,
/// especially if you know how deque DP trick works;
/// `len` and `pop` are concerned with its abstract representation, not its implementation.
/// Refer to the example below for further clarification.
/// 
/// # Query
/// - `get()` returns the minimum value in the queue, if any; otherwise returns [`None`].
/// - `most_recent()` returns the most recent value inserted into the queue.
/// 
/// # Example
/// ```
/// use ps_snippets::ds::queue_min::*;
/// 
/// let mut q = QueueMin::new(); // []
/// q.push(1);
/// q.push(3);
/// q.push(2); // [1, 3, 2]
/// assert_eq!(q.most_recent(), Some(&2));
/// assert_eq!(q.get(), Some(&1));
/// assert!(q.pop()); // [3, 2]
/// assert_eq!(q.get(), Some(&2));
/// ```
/// 
/// # Practice Problems
/// - [BOJ 11003 최솟값 찾기](https://www.acmicpc.net/problem/11003)
/// - [BOJ 10129 작은 새](https://www.acmicpc.net/problem/10129)
pub mod queue_min;
