//! Matrix utilities.

/// Square matrices.
/// 
/// # Initialization
/// - `new(n)` creates an empty `n * n` matrix.
/// - `from_list(vals, n)` creates a matrix out of `n * n` values.
/// - `eye(n)` creates an identity `n * n` matrix ($I$).
/// 
/// # Operations
/// - Output.
/// - Negation, addition, and subtraction in $O(n^2)$.
/// - Multiplication in $O(n^3)$.
/// - `pow(k)` returns the `k`-th power in $O(n^3 \log k)$.
/// 
/// # Caution
/// - `from_list` panics if `vals.len() != n*n`.
/// - Arithmetic operators panic if the sizes do not match.
/// 
/// # Examples
/// TODO
/// 
/// # Practice Problems
/// - [LC Pow of Matrix](https://judge.yosupo.jp/problem/pow_of_matrix)
///   `n <= 500, k <= 10e18`, in 1.6s
pub mod square_matrix;

/// Gaussian elimination.
/// 
/// This requires [`SquareMatrix`](square_matrix::SquareMatrix),
/// so you have to copy-paste the code for it as well if you want to use this
/// for online judges.
/// 
/// [`ElementaryOp`](gauss_elim::ElementaryOp)
/// is an enum of [elementary row operations](https://en.wikipedia.org/wiki/Elementary_matrix#Operations):
/// - `Swap(i, j)` swaps the `i`-th and `j`-th row.
/// - `Mul(i, k)` multiplies the `i`-th row by `k`.
/// - `Add(fr, k, to)` adds `k` times the `fr`-th row to the `to`-th row.
/// 
/// The following methods are added to [`SquareMatrix`](square_matrix::SquareMatrix):
/// - `apply_row_op(op)` applies the given elementary operation in $O(n)$.
/// - `make_ref()` turns the matrix into a [row-echelon form](https://en.wikipedia.org/wiki/Row_echelon_form),
///   and returns the determinant in $O(n^3)$.
/// 
/// # Caution
/// `apply_row_op` panics if the operation is invalid:
/// - `Swap(i, j)` where `i == j`.
/// - `Mul(i, k)` where `k == T::default()`.
/// - `Add(fr, k, to)` where `fr == to`.
/// 
/// # Examples
/// TODO
/// 
/// # Practice Problems
/// - [LC Determinant of Matrix](https://judge.yosupo.jp/problem/matrix_det)
///   `n <= 500` in 0.1s
pub mod gauss_elim;

/// TODO
/// 
/// # Practice Problems
/// - [LC Characteristic Polynomial](https://judge.yosupo.jp/problem/characteristic_polynomial)
///   `n <= 500` in 0.3s
pub mod char_poly;
