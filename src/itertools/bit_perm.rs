pub mod bit_perm {
	pub fn next_bit_perm(v: u32) -> u32 {
		assert!(v != 0);
		let t = v | (v-1);
		(t+1) | (((!t&(t+1))-1) >> (v.trailing_zeros()+1))
	}
	
	pub fn bit_perms(zeros: u32, ones: u32)
	-> Box<dyn Iterator<Item = u32>> {
		assert!(zeros.saturating_add(ones) < 32); // TODO: support 32
		let start = (1u32 << ones)-1;
		if start == 0 { return Box::new(std::iter::once(0)); }
		let end = start << zeros;
		Box::new(std::iter::successors(
			Some(start),
			|v| Some(next_bit_perm(*v))
		).take_while(move |v| *v <= end))
	}
	
	pub fn bitmasks_sorted_by_ones(digits: u32)
	-> impl Iterator<Item = u32> {
		assert!(digits < 32); // TODO: support 32?
		(0..=digits).flat_map(move |x| bit_perms(digits-x, x))
	}
} pub use bit_perm::{next_bit_perm, bit_perms, bitmasks_sorted_by_ones};
