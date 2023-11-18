// Remove this line when copy-pasting
use super::square_matrix::*;

pub mod gauss_elim_mod {
	use std::ops::Div;
	use super::square_matrix_mod::{Semiring, *};

	pub trait Field: Semiring + Div<Output=Self> {}
	impl<T> Field for T where T: Semiring + Div<Output=Self> {}

	pub enum ElementaryOp<T> {
		Swap(usize, usize), // R[i] <-> R[j]
		Mul(usize, T), // R[i] *= k
		Add(usize, T, usize), // R[j] += R[i] * k
	}
	use ElementaryOp::*;

	impl<T> SquareMatrix<T> where T: Semiring {
		pub fn apply_row_op(&mut self, op: ElementaryOp<T>) { match op {
			Swap(i, j) => {
				assert_ne!(i, j, "Bad row-swap");
				self.mat.swap(i, j);
			}
			Mul(i, k) => {
				assert!(k != T::default(), "Bad row-mul");
				for x in &mut self[i] { *x *= k.clone(); }
			}
			Add(fr, k, to) => {
				assert_ne!(fr, to, "Bad row-add");
				let row = std::mem::take(&mut self.mat[fr]);
				for (x, y) in self.mat[to].iter_mut().zip(row.iter()) {
					*x += k.clone() * y.clone();
				}
				self.mat[fr] = row;
			}
		}}
	}

	impl<T: Field> SquareMatrix<T> {
		pub fn make_ref(&mut self) -> T {
			let n = self.n();
			let mut istart = 0;
			let mut det: T = 1.into();
			for j in 0..n {
				// Find and swap pivot
				let mut piv = istart;
				while piv < n && self[piv][j] == T::default() { piv += 1; }
				if piv == n { continue; }
				if istart != piv {
					det = -det; self.apply_row_op(Swap(istart, piv));
				}
				// Zero the other rows
				let v = self.mat[istart][j].clone();
				det *= v.clone();
				self.apply_row_op(Mul(istart, T::from(1) / v));
				for i in istart+1..n {
					let v = self.mat[i][j].clone();
					self.apply_row_op(Add(istart, -v, i));
				}
				istart += 1;
			}
			for i in 0..n { det *= self.mat[i][i].clone(); }
			det
		}
	}
} pub use gauss_elim_mod::ElementaryOp;

