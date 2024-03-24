mod id_mapper {
	use std::collections::HashMap;
	use std::hash::Hash;

	#[derive(Debug, PartialEq, Eq, Clone)]
	pub struct IdMapper<T: Hash + Eq> { map: HashMap<T, usize> }
	impl<T: Hash + Eq> IdMapper<T> {
		pub fn new() -> Self { Self { map: HashMap::new() } }
		pub fn len(&self) -> usize { self.map.len() }
		pub fn is_empty(&self) -> bool { self.map.is_empty() }
		pub fn keys(&self) -> impl Iterator<Item = &T> { self.map.keys() }

		pub fn get(&mut self, v: T) -> usize {
			let n = self.len();
			*self.map.entry(v).or_insert(n)
		}
		pub fn get_existing(&self, v: &T) -> usize { self.map[v] }
	}
} pub use id_mapper::IdMapper;
