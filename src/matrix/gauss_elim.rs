// Remove this line when copy-pasting
use super::square_matrix::*;

pub mod gauss_elim_mod {
	use std::ops::Div;
	use super::square_matrix_mod::{Semiring, *};

	pub trait Field: Semiring + Div<Output=Self> {}
	impl<T> Field for T where T: Semiring + Div<Output=Self> {}

	impl<T> SquareMatrix<T> where T: Semiring {
		pub fn swap_row(&mut self, i: usize, j: usize) -> bool {
			if i == j { false }
			else { self.mat.swap(i, j); true }
		}
		pub fn swap_col(&mut self, i: usize, j: usize) -> bool {
			if i == j { false }
			else { for row in &mut self.mat { row.swap(i, j); } true }
		}
		pub fn mul_row(&mut self, i: usize, k: &T) {
			for x in &mut self[i] { *x *= k.clone(); }
		}
		pub fn mul_col(&mut self, j: usize, k: &T) {
			for row in &mut self.mat { row[j] *= k.clone(); }
		}
		pub fn add_row(&mut self, fr: usize, k: &T, to: usize) {
			assert_ne!(fr, to, "Bad row-add");
			let row = std::mem::take(&mut self.mat[fr]);
			for (x, y) in self.mat[to].iter_mut().zip(row.iter()) {
				*x += k.clone() * y.clone();
			}
			self.mat[fr] = row;
		}
		pub fn add_col(&mut self, fr: usize, k: &T, to: usize) {
			assert_ne!(fr, to, "Bad col-add");
			for row in &mut self.mat {
				let v = row[fr].clone();
				row[to] += k.clone() * v;
			}
		}
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
					det = -det; self.swap_row(istart, piv);
				}
				// Zero the other rows
				let v = self.mat[istart][j].clone();
				det *= v.clone();
				self.mul_row(istart, &(T::from(1) / v));
				for i in istart+1..n {
					let v = self.mat[i][j].clone();
					self.add_row(istart, &-v, i);
				}
				istart += 1;
			}
			for i in 0..n { det *= self.mat[i][i].clone(); }
			det
		}
	}
}

