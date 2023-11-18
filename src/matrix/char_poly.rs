// Remove this line when copy-pasting
use super::{square_matrix::*, gauss_elim::*};

pub mod char_poly_mod {
	use super::{SquareMatrix, ElementaryOp::{self, *}};
	use super::gauss_elim_mod::Field;

	impl<T: Field> SquareMatrix<T> {
		pub fn apply_similar(&mut self, op: ElementaryOp<T>) { match op {
			Swap(i, j) => {
				assert_ne!(i, j, "Bad row-swap");
				for row in &mut self.mat { row.swap(i, j); }
			}
			Mul(_i, k) => {
				assert!(k != T::default(), "Bad row-mul");
				todo!("Not used for Hessenberg")
			}
			Add(fr, k, to) => {
				assert_ne!(fr, to, "Bad row-add");
				for row in &mut self.mat {
					let v = row[to].clone();
					row[fr] += -k.clone() * v;
				}
			}
		}}

		pub fn make_hessenberg(&mut self) {
			let n = self.n();
			for piv in 1..n {
				// find pivot and swap
				let mut i = piv;
				while i < n {
					if self[i][piv-1] != T::default() { break; }
					i += 1;
				}
				if i == n { continue; }
				if i != piv {
					self.apply_row_op(Swap(i, piv));
					self.apply_similar(Swap(i, piv));
				}
				// zero other rows
				let x = T::from(1) / self[piv][piv-1].clone();
				for i in i+1..n {
					let mul = self[i][piv-1].clone() * x.clone();
					self.apply_row_op(Add(piv, -mul.clone(), i));
					self.apply_similar(Add(piv, -mul, i));
				}
			}
		}

		// returns det(xI + A)
		pub fn hessenberg_char_poly(&self) -> Vec<T> {
			let mut dets = vec![vec![T::from(1)]];
			let mut det = vec![T::from(1)];
			let n = self.n();
			for k in 0..n {
				let mut q = vec![T::default()];
				q.append(&mut det.clone());
				for (x, y) in q.iter_mut().zip(det.iter()) {
					*x += self[k][k].clone() * y.clone();
				}
				det = q;
				let mut prod = T::from(1);
				for i in (0..k).rev() {
					prod *= -self[i+1][i].clone();
					let z = self[i][k].clone() * prod.clone();
					let add = &dets[i];
					for (x, y) in det.iter_mut().zip(add.iter()) {
						*x += y.clone() * z.clone();
					}
				}
				dets.push(det.clone());
			}
			det
		}
	}
}
