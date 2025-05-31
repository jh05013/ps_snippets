//! Under construction (usable though).

#[derive(Debug, Clone, Default)]
pub struct Hungarian {
    mat: Vec<Vec<i64>>,
    perm: Vec<usize>,
    inv_perm: Vec<usize>,

    /// One half of the potential.
    u: Vec<i64>,
    /// The other half of the potential.
    v: Vec<i64>,
}

impl Hungarian {
    pub fn solve(mat: Vec<Vec<i64>>) -> Self {
        let mut solver = Self::default();
        for row in mat {
            solver.add_row(row);
        }
        solver
    }

    pub fn height(&self) -> usize {
        self.u.len()
    }

    pub fn width(&self) -> usize {
        self.v.len()
    }

    pub fn cost(&self) -> i64 {
        self.mat
            .iter()
            .zip(self.perm.iter())
            .map(|(row, j)| row[*j])
            .sum::<i64>()
    }

    pub fn permutation(&self) -> &[usize] {
        &self.perm
    }

    pub fn potentials(&self) -> (&[i64], &[i64]) {
        (&self.u, &self.v)
    }

    /// TODO: handle negative weights
    pub fn add_row(&mut self, row: Vec<i64>) {
        let (n, mut m) = (self.height(), self.width());
        if n == 0 {
            // initialization
            m = row.len();
            self.v = vec![0; m];
            self.inv_perm = vec![usize::MAX; m + 1];
        } else {
            // dimension check
            assert!(n <= m, "too many rows");
            assert_eq!(m, row.len(), "matrix must be rectangular");
        }
        self.mat.push(row);
        self.perm.push(0);
        self.u.push(0);

        let mut minv = vec![i64::MAX; m];
        let mut augment = vec![0; m];
        let mut used = vec![false; m + 1];
        self.inv_perm[m] = n;
        let mut j_cur = m;
        loop {
            let i = self.inv_perm[j_cur];
            if i == usize::MAX {
                break;
            }
            let row = &self.mat[i];
            let ui = self.u[i];

            used[j_cur] = true;
            // find a new reachable one
            let (delta, j_next) = used
                .iter()
                .zip(row)
                .zip(&self.v)
                .zip(&mut minv)
                .enumerate()
                .filter(|(_, (((used, _), _), _))| !**used)
                .map(|(j, (((_, rj), vj), mvj))| {
                    let diff = *rj - ui - vj;
                    if diff < *mvj {
                        *mvj = diff;
                        augment[j] = j_cur;
                    }
                    (*mvj, j)
                })
                .min()
                .unwrap();

            // recalculate potential
            for j in 0..m {
                if used[j] {
                    self.u[self.inv_perm[j]] += delta;
                    self.v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            self.u[n] += delta;

            // continue
            j_cur = j_next;
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
