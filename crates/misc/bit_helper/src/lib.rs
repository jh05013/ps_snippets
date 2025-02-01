//! Various bit functions.

pub trait BitHelper: Sized {
	// BIT LOOKUPS

	/// Returns `true` if the `b`-th bit from the right is 1.
	/// ```
	/// # use bit_helper::BitHelper;
	/// let n = 12u8; // 00001100
	/// assert!(n.bit_is_one(2));
	/// assert!(!(n.bit_is_one(4)));
	/// ```
	fn bit_is_one(&self, b: u32) -> bool;

	/// Returns the bit positions from the right that are 1,
	/// in order.
	/// ```
	/// # use bit_helper::BitHelper;
	/// let n = 12u8; // 00001100
	/// assert_eq!(n.one_bits(), vec![2, 3]);
	/// ```
	fn one_bits(&self) -> Vec<u32>;

	// BIT PERMUTATIONS

	/// Returns the number whose first `ones` bits are 1
	/// and the rest 0.
	/// 
	/// It's a bit more complex than `(1 << ones) - 1`
	/// because `ones == Self::BITS` is possible.
	/// 
	/// ⚠️ Panics if `ones > Self::BITS`.
	/// ```
	/// # use bit_helper::BitHelper;
	/// let n = u8::first_bit_perm(5); // 00011111
	/// assert_eq!(n, 31);
	/// assert_eq!(u8::first_bit_perm(8), u8::MAX);
	/// ```
	fn first_bit_perm(ones: u32) -> Self;

	/// Returns the next number that has the same number
	/// of 1 bits, if it exists. Otherwise returns `None`.
	/// ```
	/// # use bit_helper::BitHelper;
	/// let n = 22u8; // 00010110
	/// assert_eq!(n.next_bit_perm(), Some(0b11001));
	/// let n = 240u8; // 11110000
	/// assert_eq!(n.next_bit_perm(), None);
	/// ```
	fn next_bit_perm(&self) -> Option<Self>;

	/// Returns all nubmers that has `digits - ones`
	/// 0 bits and `ones` 1 bits, in increasing order.
	/// 
	/// [Source](http://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation).
	/// 
	/// ⚠️ Panics if not `ones <= digits <= Self::BITS`.
	/// ```
	/// # use bit_helper::BitHelper;
	/// let mut bits = u8::bit_perms(4, 2);
	/// assert_eq!(bits.next(), Some(3));  // 0011
	/// assert_eq!(bits.next(), Some(5));  // 0101
	/// assert_eq!(bits.next(), Some(6));  // 0110
	/// assert_eq!(bits.next(), Some(9));  // 1001
	/// assert_eq!(bits.next(), Some(10)); // 1010
	/// assert_eq!(bits.next(), Some(12)); // 1100
	/// assert_eq!(bits.next(), None);
	/// 
	/// assert_eq!(u8::bit_perms(8, 0).count(), 1);
	/// assert_eq!(u8::bit_perms(8, 1).count(), 8);
	/// assert_eq!(u8::bit_perms(8, 8).count(), 1);
	/// ```
	fn bit_perms(digits: u32, ones: u32)
		-> Box<dyn Iterator<Item = Self>>;
}

macro_rules! impl_bit_perm { ($($T:ty) *) => { $(
impl BitHelper for $T {
	fn bit_is_one(&self, b: u32) -> bool {
		self & (1 << b) != 0
	}

	fn one_bits(&self) -> Vec<u32> {
		(0..Self::BITS).filter(|b|
			self.bit_is_one(*b)
		).collect()
	}

	fn first_bit_perm(ones: u32) -> Self {
		assert!(
			ones <= Self::BITS,
			"called first_bit_perm with {} ones",
			ones
		);
		if ones == Self::BITS {
			return Self::MAX;
		}
		(1 << ones) - 1
	}

	fn next_bit_perm(&self) -> Option<Self> {
		let t = self | (self - 1);
		if t == Self::MAX {
			return None;
		}
		let z = (!t & (t + 1)) - 1;
		let z = z >> (self.trailing_zeros() + 1);
		Some((t + 1) | z)
	}

	fn bit_perms(digits: u32, ones: u32)
	-> Box<dyn Iterator<Item = Self>> {
		assert!(ones <= digits);
		assert!(digits <= Self::BITS);

		let first = Self::first_bit_perm(ones);
		if ones == 0 {
			return Box::new(std::iter::once(first));
		}
		let last = first << (digits - ones);
		if digits == ones {
			return Box::new(std::iter::once(first));
		}

		let it = std::iter::successors(
			Some(first),
			|v| v.next_bit_perm()
		).take_while(move |v| v <= &last);
		Box::new(it)
	}
}
)* }; }
impl_bit_perm!(u8 u16 u32 u64 u128 usize);