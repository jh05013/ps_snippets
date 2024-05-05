pub mod formulae_mod {
	use std::ops::*;

	pub trait Int: Clone + From<u32>
		+ Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign
		+ Mul<Output=Self> + MulAssign + Div<Output=Self> + DivAssign
		+ PartialEq + Eq + PartialOrd + Ord
	{
		#[inline] fn n0() -> Self { Self::from(0) }
		#[inline] fn n1() -> Self { Self::from(1) }
	}
	impl<T> Int for T
	where T: Clone + From<u32>
		+ Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign
		+ Mul<Output=Self> + MulAssign + Div<Output=Self> + DivAssign
		+ PartialEq + Eq + PartialOrd + Ord
	{}

	pub fn comb<T: Int>(mut n: T, k: T) -> T {
		let (mut ans, mut i) = (T::n1(), T::n0());
		while i != k {
			i += T::n1();
			ans = ans*n.clone()/i.clone();
			n -= T::n1();
		}
		ans
	}
} pub use formulae_mod::comb;
