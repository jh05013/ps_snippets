pub mod matrix {
	use std::{ops::*, fmt::*, mem::take};

	#[derive(Debug, Clone, Eq, PartialEq, Default)]
	pub struct Matrix<T> { mat: Vec<Vec<T>> }

	impl<T> Matrix<T> {
		pub fn new(n: usize, m: usize) -> Self where T: Clone + Default {
			Self { mat: vec![vec![T::default(); m]; n] }
		}
		pub fn from_line(v: Vec<T>, n: usize, m: usize) -> Self where T: Clone {
			assert!(v.len() == n*m);
			Self { mat: v.chunks(m).map(|ch| ch.to_vec()).collect::<Vec<_>>() }
		}

		#[inline] pub fn n(&self) -> usize { self.mat.len() }
		#[inline] pub fn m(&self) -> usize { self.mat[0].len() }
		#[inline] pub fn nm(&self) -> (usize, usize) { (self.n(), self.m()) }

		#[inline] pub fn elements(&self) -> impl Iterator<Item = &T> {
			self.mat.iter().flat_map(|row| row.iter())
		}
		#[inline] pub fn into_elements(self) -> impl Iterator<Item = T> {
			self.mat.into_iter().flat_map(|row| row.into_iter())
		}
		#[inline] pub fn elements_mut(&mut self) -> impl Iterator<Item = &mut T> {
			self.mat.iter_mut().flat_map(|row| row.iter_mut())
		}

		pub fn append(&mut self, mut b: Self) {
			assert_eq!(self.n(), b.n(), "Cannot append matrices of different height");
			self.mat.iter_mut().zip(b.mat.iter_mut())
				.for_each(|(row1, row2)| row1.append(row2));
		}
	}

	// io
	impl<T: Display> Display for Matrix<T> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			for row in &self.mat {
				for x in row { write!(f, "{} ", x)?; }
				writeln!(f)?;
			}
			Ok(())
		}
	}

	// indexing
	impl<T> Index<usize> for Matrix<T> { type Output = [T];
		fn index(&self, i: usize) -> &Self::Output { &self.mat[i] }
	}
	impl<T> IndexMut<usize> for Matrix<T> {
		fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.mat[i] }
	}

	// arithmetic
	impl<T: AddAssign> AddAssign for Matrix<T> {
		fn add_assign(&mut self, b: Self) {
			let (n, m) = self.nm();
			assert_eq!((n,m), b.nm(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements()).for_each(|(x, y)| *x += y);
		}
	}
	impl<T: Add<Output = T> + Default> Add for Matrix<T> { type Output = Self;
		fn add(mut self, b: Self) -> Self {
			let (n, m) = self.nm();
			assert_eq!((n,m), b.nm(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements())
				.for_each(|(x, y)| *x = take(x) + y);
			self
		}
	}
	impl<T: SubAssign> SubAssign for Matrix<T> {
		fn sub_assign(&mut self, b: Self) {
			let (n, m) = self.nm();
			assert_eq!((n,m), b.nm(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements()).for_each(|(x, y)| *x -= y);
		}
	}
	impl<T: Sub<Output = T> + Default> Sub for Matrix<T> { type Output = Self;
		fn sub(mut self, b: Self) -> Self {
			let (n, m) = self.nm();
			assert_eq!((n,m), b.nm(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements())
				.for_each(|(x, y)| *x = take(x) - y);
			self
		}
	}
	impl<T: AddAssign + Mul<Output = T> + Default + Clone> MulAssign for Matrix<T> {
		fn mul_assign(&mut self, b: Self) { *self = take(self) * b; }
	}
	impl<T: AddAssign + Mul<Output = T> + Default + Clone> Mul for Matrix<T> {
		type Output = Self;
		fn mul(self, b: Self) -> Self {
			let (n, m) = self.nm();
			let (m2, o) = b.nm();
			assert_eq!(m, m2, "Cannot multiply matrices {}x{} and {}x{}", n,m,m2,o);
			let mut ans = Matrix::<T>::new(n, o);
			for (j, bj) in b.mat.iter().enumerate() {
			for (ai, si) in ans.mat.iter_mut().zip(self.mat.iter()) {
			for (aik, bjk) in ai.iter_mut().zip(bj.iter()) {
				unsafe { *aik += si.get_unchecked(j).clone() * bjk.clone(); }
			}}}
			ans
		}
	}
	impl<T: AddAssign + Mul<Output = T> + Default + Clone + From<i32>> Matrix<T> {
		pub fn pow(&self, mut k: u64) -> Self {
			let (n, m) = self.nm();
			assert_eq!(n, m, "Cannot power a matrix {}x{}", n,m);
			let mut ans = Matrix::new(n, m);
			for i in 0..n { ans[i][i] = 1.into(); }
			let mut a = self.clone();
			while k != 0 {
				if k&1 == 1 { ans*= a.clone(); }
				k>>= 1; a*= a.clone();
			}
			ans
		}
	}

	// Gaussian elimination
	impl<T> Matrix<T> where
	T: Copy + PartialEq + Ord + Default + From<i32> +
	Mul<Output = T> + Div<Output = T> + Neg<Output = T> +
	MulAssign + AddAssign {
		#[inline] pub fn elementary_mul(&mut self, i: usize, k: T) {
			for x in &mut self[i] { *x *= k; }
		}
		#[inline] pub fn elementary_swap(&mut self, i1: usize, i2: usize) {
			assert_ne!(i1, i2, "Swapped a row {} with itself", i1);
			self.mat.swap(i1, i2);
		}
		#[inline] pub fn elementary_add(&mut self, fr: usize, k: T, to: usize) {
			assert_ne!(fr, to, "Added a row {} to itself", fr);
			let row = take(&mut self.mat[fr]);
			for (x, y) in self.mat[to].iter_mut().zip(row.iter()) {
				*x += k * *y;
			}
			self.mat[fr] = row;
		}

		pub fn make_ref(&mut self) -> T {
			let (n, m) = self.nm();
			let mut istart = 0;
			let mut det: T = 1.into();
			for j in 0..m {
				let mut piv = istart;
				while piv < n && self[piv][j] == T::default() { piv += 1; }
				if piv == n { continue; }
				if istart != piv { det = -det; self.elementary_swap(istart, piv); }

				let v = self.mat[istart][j];
				det *= v;
				self.elementary_mul(istart, T::from(1) / v);
				for i in istart+1..n {
					let v = self.mat[i][j];
					self.elementary_add(istart, -v, i);
				}
				istart += 1;
			}
			for i in 0..std::cmp::min(n, m) { det *= self.mat[i][i]; }
			det
		}

		pub fn determinant(&self) -> T {
			let (n, m) = self.nm();
			assert_eq!(n, m, "Determinant inappropriate for matrix {}x{}", n, m);
			self.clone().make_ref()
		}
	}
} pub use matrix::Matrix;
