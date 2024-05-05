// Remove this line when copy-pasting
use super::{square_matrix::*, gauss_elim::*};

pub mod char_poly_mod {
	use super::SquareMatrix;
	use super::gauss_elim_mod::Field;

	impl<T: Field> SquareMatrix<T> {
		/// Transforms it into upper Hessenberg form.
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
				self.swap_row(i, piv); self.swap_col(i, piv);
				// zero other rows
				let x = T::from(1) / self[piv][piv-1].clone();
				for i in i+1..n {
					let mul = self[i][piv-1].clone() * x.clone();
					self.add_row(piv, &-mul.clone(), i);
					self.add_col(i, &mul, piv);
				}
			}
		}

		/// Transforms it into upper Hessenberg form and returns det(xI + A).
		pub fn make_hessenberg_det_xia(&mut self) -> Vec<T> {
			self.make_hessenberg();
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

		/// Returns det(Ax + B).
		// https://tistory.joonhyung.xyz/19
		pub fn det_axb(a: &mut Self, b: &mut Self) -> Vec<T> {
			let n = a.n();
			assert_eq!(n, b.n(), "Matrix sizes mismatch");

			let (mut alpha, mut beta) = (0, 0);
			let mut det = T::from(1);
			while alpha + beta < n {
				for i in 0..alpha {
					b.add_col(i, &-a[i][alpha].clone(), alpha);
					a.add_col(i, &-a[i][alpha].clone(), alpha);
				}
				for i in alpha..n-beta {
					if a[i][alpha] != T::default() {
						if a.swap_row(alpha, i) { det = -det; }
						b.swap_row(alpha, i);
						break;
					}
				}
				if a[alpha][alpha] != T::default() {
					det *= a[alpha][alpha].clone();
					let mul = T::from(1) / a[alpha][alpha].clone();
					a.mul_row(alpha, &mul); b.mul_row(alpha, &mul);
					for i in alpha+1..n {
						let add = -a[i][alpha].clone();
						b.add_row(alpha, &add, i);
						a.add_row(alpha, &add, i);
					}
					alpha += 1;
				}
				else {
					let r = n-1-beta;
					if a.swap_col(alpha, r) { det = -det; }
					b.swap_col(alpha, r);
					let mut pos = None;
					for i in (0..=r).rev() {
						if b[i][r] != T::default() { pos = Some(i); break; }
					}

					let pos = match pos {
						Some(pos) => pos, None => return vec![]
					};
					if pos < alpha {
						a.swap_row(pos, alpha-1); b.swap_row(pos, alpha-1);
						a.swap_col(pos, alpha-1); b.swap_col(pos, alpha-1);
						if a.swap_row(alpha-1, r) { det = -det; }
						b.swap_row(alpha-1, r);
						alpha -= 1;
					}
					else {
						if a.swap_row(pos, r) { det = -det; }
						b.swap_row(pos, r);
					}
					det *= b[r][r].clone();
					let mul = T::from(1) / b[r][r].clone();
					a.mul_row(r, &mul); b.mul_row(r, &mul);
					for i in 0..r {
						let add = -b[i][r].clone();
						a.add_row(r, &add, i);
						b.add_row(r, &add, i);
					}
					beta += 1;
				}
			}

			let mut c = Self::new(alpha);
			for i in 0..alpha { for j in 0..alpha {
				c[i][j] = b[i][j].clone();
			}}
			let mut poly = c.make_hessenberg_det_xia();
			poly.iter_mut().for_each(|x| *x *= det.clone());
			poly
		}
	}
}
