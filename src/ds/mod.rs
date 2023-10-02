/// A static range sum data structure.
/// 
/// # Examples
/// TODO
/// 
/// # Practice Problems
/// - [LC Static Range Sum](https://judge.yosupo.jp/problem/static_range_sum)
/// - [BOJ 16139 인간-컴퓨터 상호작용](https://www.acmicpc.net/problem/16139)
pub mod range_sum;

/// A lazy segment tree which also works as a Segment Tree Beats.
/// 
/// To use it, first create a `Node` struct (e.g. `Node(u32)`)
/// and implement the `LazyMonoid` trait:
/// - `type Lazy`: the type of the lazy operation.
/// - `ID: Self`: the identity element of the value operation.
/// - `LAZY_ID: Lazy`: the identity element of the lazy operation.
/// - `op(a: Self, b: Self) -> Self`: the value operation `a * b`.
/// - `op_lazy(a: Lazy, b: Lazy) -> Lazy`: the lazy operation `a * b`.
/// - `unlazy(v: Self, size: usize, lz: Lazy) -> Option<Self>`:
///   tries to apply the lazy operation to the value `v` on
///   a node of size `size`. In a regular lazy segment tree, the result
///   is always [`Some`]. For Segment Tree Beats, the result may be
///   [`None`] if the tagging condition fails.
/// 
/// # Examples
/// TODO
/// 
/// # Practice Problems
/// - [BOJ 10999 구간 합 구하기 2](https://www.acmicpc.net/problem/10999)
/// - [LC Range Affine Range Sum](https://judge.yosupo.jp/problem/range_affine_range_sum)
/// - [BOJ 17474 수열과 쿼리 26](https://www.acmicpc.net/problem/17474) (Beats)
/// - [BOJ 17476 수열과 쿼리 28](https://www.acmicpc.net/problem/17476) (Beats)
pub mod lazy_segtree;
