mod bitset {
	const B: usize = usize::BITS as usize;
	#[derive(Clone, Debug, Default)]
	pub struct Bitset(Vec<usize>, usize);

	impl Bitset {
		pub fn new(n: usize) -> Self {
			let len = (n+B-1) / B;
			Self(vec![0; len], n)
		}
		pub fn len(&self) -> usize { self.1 }

		pub fn iter_masks(&self) -> impl Iterator<Item = &usize> {
			self.0.iter()
		}

		// (index in vector, bit in slot)
		fn single_bit(i: usize) -> (usize, usize) { (i/B, 1<<(i%B)) }

		pub fn set(&mut self, i: usize) {
			assert!(i < self.len());
			let (i, b) = Self::single_bit(i); self.0[i] |= b;
		}
		pub fn unset(&mut self, i: usize) {
			assert!(i < self.len());
			let (i, b) = Self::single_bit(i); self.0[i] &= !b;
		}
		pub fn toggle(&mut self, i: usize) {
			assert!(i < self.len());
			let (i, b) = Self::single_bit(i); self.0[i] ^= b;
		}
		pub fn get(&self, i: usize) -> bool {
			assert!(i < self.len());
			let (i, b) = Self::single_bit(i);
			self.0[i] & b != 0
		}

		pub fn count_ones(&self) -> usize {
			self.0.iter().map(|v| v.count_ones() as usize).sum::<usize>()
		}
		pub fn count_zeros(&self) -> usize {
			self.0.iter().map(|v| v.count_zeros() as usize).sum::<usize>()
		}
	}

	impl std::ops::BitAnd for &Bitset {
		type Output = Bitset;

		fn bitand(self, rhs: Self) -> Self::Output {
			assert_eq!(self.len(), rhs.len());
			let v = self.0.iter().zip(&rhs.0)
				.map(|(x,y)| x&y)
				.collect::<Vec<_>>();
			Bitset(v, self.len())
		}
	}
} use bitset::Bitset;
