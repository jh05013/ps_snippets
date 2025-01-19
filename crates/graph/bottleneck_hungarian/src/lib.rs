//! Under construction.

#[derive(Debug, Clone, Default)]
pub struct BottleneckHungarian {
	mat: Vec<Vec<i64>>,
	perm: Vec<usize>,
	inv_perm: Vec<usize>,
}

impl BottleneckHungarian {
	pub fn solve(mat: Vec<Vec<i64>>) -> Self {
		let mut solver = Self::default();
		for row in mat { solver.add_row(row); }
		solver
	}

	pub fn height(&self) -> usize {
		self.mat.len()
	}

	pub fn width(&self) -> usize {
		if self.mat.is_empty() {
			return 0;
		}
		self.mat[0].len()
	}

	pub fn cost(&self) -> i64 {
		self.mat.iter().zip(self.perm.iter())
			.map(|(row, j)| row[*j])
			.max()
			.unwrap()
	}

	pub fn permutation(&self) -> &[usize] {
		&self.perm
	}

	pub fn add_row(&mut self, row: Vec<i64>) {
		let (n, mut m) = (self.height(), self.width());
		if n == 0 {
			// initialization
			m = row.len();
			self.inv_perm = vec![usize::MAX; m+1];
		}
		else {
			// dimension check
			assert!(n <= m, "too many rows");
			assert_eq!(m, row.len(), "matrix must be rectangular");
		}
		self.mat.push(row);
		self.perm.push(0);

		let mut minv = vec![i64::MAX; m];
		let mut augment = vec![0; m];
		let mut used = vec![false; m+1];
		self.inv_perm[m] = n;
		let mut j_cur = m;
		loop {
			let i = self.inv_perm[j_cur];
			if i == usize::MAX { break; }
			let row = &self.mat[i];

			used[j_cur] = true;
			// find a new reachable one
			j_cur = used.iter()
				.zip(row)
				.zip(&mut minv)
				.enumerate()
				.filter(|(_, ((used, _), _))| !**used)
				.map(|(j, ((_, rj), mvj))| {
					if *rj < *mvj {
						*mvj = *rj;
						augment[j] = j_cur;
					}
					(*mvj, j)
				})
				.min()
				.unwrap()
				.1;
		}

		// apply augmenting paths
		while j_cur != m {
			let j_prev = augment[j_cur];
			self.inv_perm[j_cur] = self.inv_perm[j_prev];
			self.perm[self.inv_perm[j_cur]] = j_cur;
			j_cur = j_prev;
		}
	}
}
