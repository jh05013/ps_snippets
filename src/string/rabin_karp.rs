pub mod rabin_karp_mod {
	use std::collections::VecDeque;
	use std::collections::hash_map::DefaultHasher;
	use std::hash::Hasher;

	const MOD: u64 = 9_223_372_036_854_775_783;
	const A: u64 = 2;
	const IA: u64 = (MOD+1)/2;
	fn mul64(a: u64, b: u64) -> u64 {
		let x = u128::from(a)*u128::from(b)%u128::from(MOD);
		u64::try_from(x).unwrap()
	}

	pub struct RkBase { base: Vec<u64> }

	impl RkBase {
		pub fn new() -> Self {
			let mut rng = DefaultHasher::new();
			let base = (0..256).map(|i| {
				rng.write_usize(i); rng.finish() % MOD
			}).collect::<Vec<_>>();
			Self { base }
		}
		pub fn hash(&self, c: char) -> u64 {
			self.base[usize::try_from(u32::from(c)).unwrap()]
		}

		pub fn spawn(&self) -> RkHash<'_> {
			RkHash { base: self, deque: VecDeque::new(), hash: 0, lead: 1 }
		}
	}

	pub struct RkHash<'a> {
		base: &'a RkBase,
		deque: VecDeque<char>,
		hash: u64,
		lead: u64,
	}
	impl<'a> RkHash<'a> {
		pub fn len(&self) -> usize { self.deque.len() }
		pub fn get(&self) -> u64 { self.hash }
		fn hash(&self, c: char) -> u64 { self.base.hash(c) }

		pub fn push_back(&mut self, c: char) {
			self.deque.push_back(c);
			self.hash = ((self.hash * A)%MOD + self.hash(c))%MOD;
			self.lead = (self.lead * A)%MOD;
		}
		pub fn push_front(&mut self, c: char) {
			self.deque.push_front(c);
			self.hash = (self.hash + mul64(self.lead, self.hash(c))) % MOD;
			self.lead = (self.lead * A)%MOD;
		}
		pub fn pop_back(&mut self) -> Option<char> {
			let c = self.deque.pop_back()?;
			self.hash = (self.hash + MOD - self.base.hash(c))%MOD;
			self.hash = mul64(self.hash, IA);
			self.lead = mul64(self.lead, IA);
			Some(c)
		}
		pub fn pop_front(&mut self) -> Option<char> {
			let c = self.deque.pop_front()?;
			self.lead = mul64(self.lead, IA);
			let to_sub = mul64(self.lead, self.base.hash(c));
			self.hash = (self.hash + MOD - to_sub) % MOD;
			Some(c)
		}
	}
} pub use rabin_karp_mod::{RkBase, RkHash};
