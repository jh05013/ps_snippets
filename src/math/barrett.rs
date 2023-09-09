//! Barrett reduction, a fast modulo by a constant when such a
//! constant is not known at compile time.

/// A Barrett reduction interface.
/// 
/// # Examples
/// ```
/// use ps_snippets::math::barrett::*;
/// 
/// let bar = Barrett::new(1000);
/// assert_eq!(bar.reduce(1234567), 567);
/// assert_eq!(bar.modpow(2, 20), 576); // 1048576
/// ```
/// 
/// # Practice Problems
/// TODO
/// 
#[derive(Debug, Clone)]
pub struct Barrett { n: u64, m: u128 }

impl Barrett {
	
	/// Creates a new Barrett reduction interface modulo `n`.
	/// 
	/// ⚠️ Panics from division by 0 if `n == 0`.
	pub fn new(n: u64) -> Self {
		Barrett { n, m: (1u128 << 64) / n as u128 }
	}

	/// Returns `x` mod `n`.
	pub fn reduce(&self, x: u64) -> u64 {
		let q = ((self.m * (x as u128)) >> 64) as u64;
		let r = x - q * self.n;
		if r >= self.n { r - self.n } else { r }
	}

	/// Returns $x^k$ mod `n`.
	/// 
	/// 🕒 $O(\log k)$
	pub fn modpow(&self, mut x: u64, mut k: u64) -> u64 {
		let mut ans = 1u64;
		while k != 0 {
			if (k&1) != 0 { ans = self.reduce(ans * x); }
			k>>= 1; x = self.reduce(x * x);
		}
		ans
	}
}
