pub mod square_matrix_mod {
	use std::{ops::*, fmt::*, mem::take};

	pub trait Semiring:
		Neg<Output=Self>
		+ AddAssign + Add<Output=Self> + MulAssign + Mul<Output=Self>
		+ Clone + Default + From<i32> + PartialEq + Display + Debug {}
	impl<T> Semiring for T where T:
		Neg<Output=Self>
		+ AddAssign + Add<Output=Self> + MulAssign + Mul<Output=Self>
		+ Clone + Default + From<i32> + PartialEq + Display + Debug {}

	/// A square matrix.
	#[derive(Debug, Clone, Eq, PartialEq, Default)]
	pub struct SquareMatrix<T> { pub mat: Vec<Vec<T>> }

	impl<T> SquareMatrix<T> {
		/// Creates an empty `n * n` matrix.
		pub fn new(n: usize) -> Self where T: Clone + Default {
			Self { mat: vec![vec![T::default(); n]; n] }
		}
		/// Cretaes a matrix out of `n * n` values.
		pub fn from_line(n: usize, vals: &[T]) -> Self where T: Clone {
			assert!(vals.len() == n*n,
				"{}x{} matrix cannot be made from {} values", n, n, vals.len()
			);
			if n == 0 { return Self { mat: vec![] }; }
			Self { mat: vals.chunks(n).map(<[T]>::to_vec).collect() }
		}
		/// Creates an identity `n * n` matrix.
		pub fn eye(n: usize) -> Self where T: Clone + Default + From<i32> {
			let mut mat = Self::new(n);
			for i in 0..n { mat[i][i] = 1.into(); }
			mat
		}

		/// Returns `n`.
		pub fn n(&self) -> usize { self.mat.len() }

		/// Iterates over its elements.
		pub fn elements(&self) -> impl Iterator<Item = &T> {
			self.mat.iter().flat_map(|row| row.iter())
		}
		/// Iterates over its owned elements.
		pub fn into_elements(self) -> impl Iterator<Item = T> {
			self.mat.into_iter().flat_map(IntoIterator::into_iter)
		}
		/// Iterates mutably over its elements.
		pub fn elements_mut(&mut self) -> impl Iterator<Item = &mut T> {
			self.mat.iter_mut().flat_map(|row| row.iter_mut())
		}
	}

	// io
	impl<T: Display> Display for SquareMatrix<T> {
		fn fmt(&self, f: &mut Formatter) -> Result {
			for row in &self.mat {
				for x in row { write!(f, "{x} ")?; }
				writeln!(f)?;
			}
			Ok(())
		}
	}

	// indexing
	/// Returns the `i`-th row.
	impl<T> Index<usize> for SquareMatrix<T> { type Output = [T];
		fn index(&self, i: usize) -> &Self::Output { &self.mat[i] }
	}
	/// Returns the `i`-th row mutably.
	impl<T> IndexMut<usize> for SquareMatrix<T> {
		fn index_mut(&mut self, i: usize) -> &mut Self::Output { &mut self.mat[i] }
	}

	// arithmetic
	impl<T: Neg<Output = T> + Default> Neg for SquareMatrix<T> {
		type Output = Self;
		fn neg(mut self) -> Self::Output {
			self.elements_mut().for_each(|x| *x = -take(x));
			self
		}
	}
	impl<T: AddAssign> AddAssign for SquareMatrix<T> {
		fn add_assign(&mut self, b: Self) {
			assert_eq!(self.n(), b.n(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements()).for_each(|(x, y)| *x += y);
		}
	}
	impl<T: Add<Output = T> + Default> Add for SquareMatrix<T> { type Output = Self;
		fn add(mut self, b: Self) -> Self {
			assert_eq!(self.n(), b.n(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements())
				.for_each(|(x, y)| *x = take(x) + y);
			self
		}
	}
	impl<T: SubAssign> SubAssign for SquareMatrix<T> {
		fn sub_assign(&mut self, b: Self) {
			assert_eq!(self.n(), b.n(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements()).for_each(|(x, y)| *x -= y);
		}
	}
	impl<T: Sub<Output = T> + Default> Sub for SquareMatrix<T> { type Output = Self;
		fn sub(mut self, b: Self) -> Self {
			assert_eq!(self.n(), b.n(), "Matrix sizes mismatch");
			self.elements_mut().zip(b.into_elements())
				.for_each(|(x, y)| *x = take(x) - y);
			self
		}
	}
	impl<T: MulAssign + Clone> MulAssign<T> for SquareMatrix<T> {
		fn mul_assign(&mut self, k: T) {
			self.elements_mut().for_each(|x| *x *= k.clone());
		}
	}
	impl<T: Mul<Output = T> + Default + Clone> Mul<T> for SquareMatrix<T> {
		type Output = Self;
		fn mul(mut self, k: T) -> Self {
			self.elements_mut().for_each(|x| *x = take(x) * k.clone());
			self
		}
	}
	impl<T: Semiring> MulAssign for SquareMatrix<T> {
		fn mul_assign(&mut self, b: Self) { *self = take(self) * b; }
	}
	impl<T: Semiring> Mul for SquareMatrix<T> {
		type Output = Self;
		fn mul(self, b: Self) -> Self {
			assert_eq!(self.n(), b.n(), "Matrix sizes mismatch");
			let mut ans = Self::new(self.n());
			for (j, bj) in b.mat.iter().enumerate() {
			for (ai, si) in ans.mat.iter_mut().zip(self.mat.iter()) {
			for (aik, bjk) in ai.iter_mut().zip(bj.iter()) {
				// SAFETY: j < b.n == self.n
				unsafe { *aik += si.get_unchecked(j).clone() * bjk.clone(); }
			}}}
			ans
		}
	}
	impl<T: Semiring> SquareMatrix<T> {
		/// Returns its `k`-th power.
		pub fn pow(&self, mut k: u64) -> Self {
			let mut ans = Self::eye(self.n());
			let mut a = self.clone();
			while k != 0 {
				if k&1 == 1 { ans*= a.clone(); }
				k>>= 1; a*= a.clone();
			}
			ans
		}
	}
} pub use square_matrix_mod::SquareMatrix;
