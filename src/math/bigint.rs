
#[allow(clippy::unreadable_literal)]
pub mod fft_mod {
	pub trait FftBase {
		type Ty: Copy + Default;
		fn root(idx: usize, invert: bool) -> Self::Ty;
		fn inv_p2(idx: usize) -> Self::Ty;
		fn add(a: Self::Ty, b: Self::Ty) -> Self::Ty;
		fn sub(a: Self::Ty, b: Self::Ty) -> Self::Ty;
		fn mul(a: Self::Ty, b: Self::Ty) -> Self::Ty;
	}

	pub struct Mod998244353;
	impl FftBase for Mod998244353 {
		type Ty = u32;
		fn root(idx: usize, invert: bool) -> u32 {
			if invert { [1, 998244352, 86583718, 509520358, 337190230, 87557064, 609441965, 135236158, 304459705, 685443576, 381598368, 335559352, 129292727, 358024708, 814576206, 708402881, 283043518, 3707709, 121392023, 704923114, 950391366, 428961804, 382752275, 469870224][idx] }
			else { [1, 998244352, 911660635, 372528824, 929031873, 452798380, 922799308, 781712469, 476477967, 166035806, 258648936, 584193783, 63912897, 350007156, 666702199, 968855178, 629671588, 24514907, 996173970, 363395222, 565042129, 733596141, 267099868, 15311432][idx] }
		}
		fn inv_p2(idx: usize) -> u32 {
			[1, 499122177, 748683265, 873463809, 935854081, 967049217, 982646785, 990445569, 994344961, 996294657, 997269505, 997756929, 998000641, 998122497, 998183425, 998213889, 998229121, 998236737, 998240545, 998242449, 998243401, 998243877, 998244115, 998244234][idx]
		}
		fn add(a: u32, b: u32) -> u32 { (a+b) % 998244353 }
		fn sub(a: u32, b: u32) -> u32 { (a+998244353-b) % 998244353 }
		fn mul(a: u32, b: u32) -> u32 {
			u32::try_from(u64::from(a)*u64::from(b) % 998244353).unwrap()
		}
	}
	pub struct Mod1092616193;
	impl FftBase for Mod1092616193 {
		type Ty = u32;
		fn root(idx: usize, invert: bool) -> u32 {
			if invert { [1, 1092616192, 64522609, 1032520529, 339061949, 27527525, 93648825, 1033369768, 712664421, 950593389, 647547039, 179030670, 336332660, 250105162, 64481618, 495417288, 568756837, 1047590964, 767212362, 102805677, 945086739, 422582537][idx] }
			else { [1, 1092616192, 1028093584, 239571712, 1043144720, 872231331, 88368768, 223607613, 42054824, 873412670, 695854315, 669752502, 267003629, 669512101, 882027755, 347003797, 330611819, 80286801, 575983809, 918212341, 1005563392, 633127788][idx] }
		}
		fn inv_p2(idx: usize) -> u32 {
			[1, 546308097, 819462145, 956039169, 1024327681, 1058471937, 1075544065, 1084080129, 1088348161, 1090482177, 1091549185, 1092082689, 1092349441, 1092482817, 1092549505, 1092582849, 1092599521, 1092607857, 1092612025, 1092614109, 1092615151, 1092615672][idx]
		}
		fn add(a: u32, b: u32) -> u32 { (a+b) % 1092616193 }
		fn sub(a: u32, b: u32) -> u32 { (a+1092616193-b) % 1092616193 }
		fn mul(a: u32, b: u32) -> u32 {
			u32::try_from(u64::from(a)*u64::from(b) % 1092616193).unwrap()
		}
	}

	impl<T: FftBase, U: FftBase> FftBase for (T, U) {
		type Ty = (T::Ty, U::Ty);
		fn root(idx: usize, invert: bool) -> Self::Ty {
			(T::root(idx, invert), U::root(idx, invert))
		}
		fn inv_p2(idx: usize) -> Self::Ty { (T::inv_p2(idx), U::inv_p2(idx)) }
		fn add(a: Self::Ty, b: Self::Ty) -> Self::Ty { (T::add(a.0,b.0), U::add(a.1,b.1)) }
		fn sub(a: Self::Ty, b: Self::Ty) -> Self::Ty { (T::sub(a.0,b.0), U::sub(a.1,b.1)) }
		fn mul(a: Self::Ty, b: Self::Ty) -> Self::Ty { (T::mul(a.0,b.0), U::mul(a.1,b.1)) }
	}

	#[allow(clippy::many_single_char_names)]
	pub fn fft<T: FftBase>(a: &mut [T::Ty], invert: bool) {
		let n = a.len();
		let mut j = 0;
		for i in 1..n {
			let mut bit = n >> 1;
			while (j & bit) != 0 { j ^= bit; bit >>= 1; }
			j ^= bit;
			if i < j { a.swap(i, j); }
		}
		let mut len = 2;
		let mut idx = 1;
		while len <= n {
			let root = T::root(idx, invert);
			for i in 0..n/len {
				let i = i * len;
				let mut w = T::root(0, false);
				for j in 0..len/2 {
					let u = a[i+j];
					let v = T::mul(a[i+j+len/2], w);
					a[i+j] = T::add(u, v); a[i+j+len/2] = T::sub(u, v);
					w = T::mul(w, root);
				}
			}
			len <<= 1; idx += 1;
		}
		if invert {
			let inv = T::inv_p2(usize::try_from(n.trailing_zeros()).unwrap());
			for x in a { *x = T::mul(*x, inv); }
		}
	}

	pub fn polymul_assign<T: FftBase>(a: &mut Vec<T::Ty>, mut b: Vec<T::Ty>) {
		if a.is_empty() || b.is_empty() { return *a = vec![]; }
		let lens = a.len() + b.len() - 1;
		a.resize(lens.next_power_of_two(), T::Ty::default());
		b.resize(lens.next_power_of_two(), T::Ty::default());
		fft::<T>(a, false); fft::<T>(&mut b, false);
		for (x, y) in a.iter_mut().zip(&b) { *x = T::mul(*x, *y); }
		fft::<T>(a, true); a.truncate(lens);
	}

	pub fn polysquare_assign<T: FftBase>(a: &mut Vec<T::Ty>) {
		if a.is_empty() { return; }
		let lens = a.len()*2 - 1;
		a.resize(lens.next_power_of_two(), T::Ty::default());
		fft::<T>(a, false);
		for x in a.iter_mut() { *x = T::mul(*x, *x); }
		fft::<T>(a, true); a.truncate(lens);
	}

	pub fn polymul_1e18(a: &[u32], b: &[u32]) -> Vec<u64> {
		let mut ta = a.iter().map(
			|x| (*x%998244353, *x%1092616193)
		).collect::<Vec<_>>();
		let tb = b.iter().map(
			|x| (*x%998244353, *x%1092616193)
		).collect::<Vec<_>>();
		polymul_assign::<(Mod998244353, Mod1092616193)>(&mut ta, tb);
		ta.into_iter().map(|(x, y)| {
			u64::try_from((u128::from(x) * 533230094720090466 + u128::from(y) * 557467849938517664) % 1090697944658608129).unwrap()
		}).collect::<Vec<_>>()
	}

	pub fn polysquare_1e18(a: &[u32]) -> Vec<u64> {
		let mut ta = a.iter().map(
			|x| (*x%998244353, *x%1092616193)
		).collect::<Vec<_>>();
		polysquare_assign::<(Mod998244353, Mod1092616193)>(&mut ta);
		ta.into_iter().map(|(x, y)| {
			u64::try_from((u128::from(x) * 533230094720090466 + u128::from(y) * 557467849938517664) % 1090697944658608129).unwrap()
		}).collect::<Vec<_>>()
	}
} pub use fft_mod::{fft, polymul_assign, polymul_1e18, polysquare_assign, polysquare_1e18};

pub mod ubigint_mod {
	const D: usize = 5;
	const BASE: u32 = 100_000u32;

	#[derive(Clone, Debug, Default, PartialEq, Eq)]
	pub struct UbigInt(Vec<u32>);

	//////////////// Basic/helper stuff ////////////////
	impl UbigInt {
		pub fn len(&self) -> usize {
			if self.is_zero() { return 0; }
			let x = format!("{}", self.0.last().unwrap());
			self.0.len()*D - (D - x.len())
		}
		fn raw_len(&self) -> usize { self.0.len() }

		pub fn is_zero(&self) -> bool { self.0.is_empty() }

		/// Removes leading zeroes.
		fn sanitize(&mut self) {
			while self.0.last() == Some(&0) { self.0.pop(); }
		}
		/// Removes leading zeroes.
		fn sanitized(mut self) -> Self { self.sanitize(); self }

		/// `* B^d`
		fn shift_up(&mut self, d: usize) { self.0.splice(..0, vec![0; d]); }
		/// `/ B^d`
		fn shift_down(&mut self, d: usize) {
			self.0.splice(..d, []);
		}
		fn carry_on(&mut self) {
			let mut overflow = 0;
			for a in &mut self.0 {
				*a += overflow; overflow = *a / BASE; *a %= BASE;
			}
			if overflow > 0 { self.0.push(overflow); }
		}
	}
	
	//////////////// IO ////////////////
	mod io {
		use std::num::ParseIntError;
		use super::{D, UbigInt};

		impl From<u32> for UbigInt {
			fn from(value: u32) -> Self {
				let mut z = Self(vec![value]).sanitized();
				z.carry_on(); z
			}
		}

		impl std::str::FromStr for UbigInt {
			type Err = ParseIntError;
			fn from_str(s: &str) -> Result<Self, Self::Err> {
				let n = s.len();
				let mut digs: Vec<u32> = (1..=n/D).map(|i| {
					let start = n-D*i;
					s[start..start+D].parse()
				}).collect::<Result<Vec<_>, Self::Err>>()?;
				if n%D != 0 { digs.push(s[..n%D].parse()?); }
				Ok(Self(digs).sanitized())
			}
		}

		impl std::fmt::Display for UbigInt {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				if self.is_zero() { return write!(f, "0"); }
				for (i, v) in self.0.iter().rev().enumerate() {
					if i == 0 { write!(f, "{}", v)?; }
					else { write!(f, "{:05}", v)?; } // TODO don't hardcode
				}
				Ok(())
			}
		}
	}

	//////////////// OPERATIONS ////////////////
	mod op {
		use std::{mem::swap, cmp::Ordering};
		use std::ops::*;
		use super::super::fft_mod::{polysquare_1e18, polymul_1e18};
		use super::{UbigInt, BASE};

		impl PartialOrd for UbigInt {
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
				Some(self.cmp(other))
			}
		}
		impl Ord for UbigInt {
			fn cmp(&self, other: &Self) -> Ordering {
				self.raw_len().cmp(&other.raw_len()).then_with(|| {
				self.0.iter().rev().zip(other.0.iter().rev())
					.map(|(a, b)| a.cmp(b)).find(|o| o.is_ne())
					.unwrap_or(Ordering::Equal)
				})
			}
		}

		impl AddAssign for UbigInt {
			fn add_assign(&mut self, mut rhs: Self) {
				if self.raw_len() < rhs.raw_len() { swap(self, &mut rhs); }
				for (a, b) in self.0.iter_mut().zip(&rhs.0) { *a += *b; }
				self.carry_on();
			}
		}
		impl Add for UbigInt { type Output = Self;
			fn add(mut self, rhs: Self) -> Self { self += rhs; self }
		}

		impl SubAssign for UbigInt {
			fn sub_assign(&mut self, mut rhs: Self) {
				assert!(self >= &mut rhs, "{:?} - {:?} < 0", self, rhs);
				if self.is_zero() { return; }
				self.0[0] += 1;
				for a in &mut self.0 { *a += BASE-1; }
				for (a, b) in self.0.iter_mut().zip(&rhs.0) { *a -= *b; }
				self.carry_on(); self.0.pop(); self.sanitize();
			}
		}
		impl Sub for UbigInt { type Output = Self;
			fn sub(mut self, rhs: Self) -> Self { self -= rhs; self }
		}

		impl MulAssign for UbigInt {
			fn mul_assign(&mut self, rhs: Self) {
				let b64 = u64::from(BASE);
				let mut digs = polymul_1e18(&self.0, &rhs.0);
				let mut overflow = 0;
				for a in &mut digs {
					*a += overflow; overflow = *a / b64; *a %= b64;
				}
				while overflow > 0 { digs.push(overflow % b64); overflow /= b64; }
				self.0 = digs.into_iter().map(|x| u32::try_from(x).unwrap()).collect();
			}
		}
		impl Mul for UbigInt { type Output = Self;
			fn mul(mut self, rhs: Self) -> Self { self *= rhs; self }
		}

		impl DivAssign for UbigInt {
			fn div_assign(&mut self, mut rhs: Self) {
				assert!(!rhs.is_zero(), "division by 0");
				if self < &mut rhs { return self.0.clear(); }

				let newton = |x: UbigInt, rhs: UbigInt, prec| {
					let l = x.clone() + x.clone();
					let mut r = x.clone(); r.square(); r *= rhs;
					r.shift_down(prec); l - r
				};

				let mut prec = 1;
				let mut inv = UbigInt::from(1);

				let mut cut_rhs = rhs.clone();
				cut_rhs.shift_down(cut_rhs.raw_len()-1);
				for _ in 0..22 {
					inv = newton(inv, cut_rhs.clone(), 1);
				}

				while prec < self.raw_len() + rhs.raw_len() {
					inv.shift_up(prec); prec += prec;
					for _ in 0..2 {
						inv = newton(inv, rhs.clone(), prec + rhs.raw_len() - 1);
					}
				}
				loop {
					let new_inv = newton(inv.clone(), rhs.clone(), prec + rhs.raw_len() - 1);
					if inv == new_inv { break; }
					inv = new_inv;
				}

				inv *= self.clone(); inv.shift_down(prec+rhs.raw_len()-1);
				*self = inv
			}
		}
		impl Div for UbigInt { type Output = Self;
			fn div(mut self, rhs: Self) -> Self { self /= rhs; self }
		}

		impl UbigInt {
			pub fn square(&mut self) {
				let b64 = u64::from(BASE);
				let mut digs = polysquare_1e18(&self.0);
				let mut overflow = 0;
				for a in &mut digs {
					*a += overflow; overflow = *a / b64; *a %= b64;
				}
				while overflow > 0 { digs.push(overflow % b64); overflow /= b64; }
				self.0 = digs.into_iter().map(|x| u32::try_from(x).unwrap()).collect();
			}

			pub fn divmod(self, rhs: Self) -> (Self, Self) {
				let mut q = self.clone(); q /= rhs.clone();
				(q.clone(), self-q*rhs)
			}
		}
	}
} pub use ubigint_mod::UbigInt;

pub mod bigint_mod {
	use super::UbigInt;

	#[derive(Clone, Debug, Default, PartialEq, Eq)]
	pub struct BigInt { minus: bool, raw: UbigInt }

	//////////////// Basic/helper stuff ////////////////
	impl BigInt {
		pub fn len(&self) -> usize { self.raw.len() }
		pub fn is_pos(&self) -> bool { !self.minus && !self.raw.is_zero() }
		pub fn is_zero(&self) -> bool { self.raw.is_zero() }
		pub fn is_neg(&self) -> bool { self.minus }
		pub fn abs(mut self) -> Self { self.minus = false; self }
	}
	
	//////////////// IO ////////////////
	mod io {
		use std::num::ParseIntError;
		use super::{BigInt, UbigInt};

		impl From<u32> for BigInt {
			fn from(value: u32) -> Self { Self { minus: false, raw: value.into() }}
		}
		impl From<i32> for BigInt {
			fn from(value: i32) -> Self { Self {
				minus: value < 0, raw: u32::try_from(value.abs()).unwrap().into()
			}}
		}
		impl From<UbigInt> for BigInt {
			fn from(value: UbigInt) -> Self { Self { minus: false, raw: value }}
		}

		impl std::str::FromStr for BigInt {
			type Err = ParseIntError;
			fn from_str(mut s: &str) -> Result<Self, Self::Err> {
				let mut minus = false;
				if s.starts_with('-') { minus = true; s = &s[1..]; }
				Ok(Self { minus, raw: s.parse()? })
			}
		}

		impl std::fmt::Display for BigInt {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				if self.minus { write!(f, "-")?; }
				write!(f, "{}", self.raw)
			}
		}
	}

	//////////////// OPERATIONS ////////////////
	mod op {
		use std::ops::*;
		use std::cmp::Ordering;
		use std::mem::swap;
		use super::{UbigInt, BigInt};

		impl PartialOrd for BigInt {
			fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
				Some(self.cmp(other))
			}
		}
		impl Ord for BigInt {
			fn cmp(&self, other: &Self) -> Ordering {
				other.minus.cmp(&self.minus).then_with(|| {
					self.raw.cmp(&other.raw)
				})
			}
		}

		impl AddAssign for BigInt {
			fn add_assign(&mut self, mut rhs: Self) {
				if self.minus == rhs.minus { return self.raw += rhs.raw; }
				if self.minus { swap(self, &mut rhs); }
				*self -= rhs.abs();
			}
		}
		impl Add for BigInt { type Output = Self;
			fn add(mut self, rhs: Self) -> Self { self += rhs; self }
		}

		impl Neg for BigInt { type Output = Self;
			fn neg(mut self) -> Self { self.negate(); self }
		}
		impl SubAssign for BigInt {
			fn sub_assign(&mut self, mut rhs: Self) {
				if self.minus != rhs.minus { return self.raw += rhs.raw; }
				if self.minus {
					swap(self, &mut rhs); self.negate(); rhs.negate();
				}
				if self < &mut rhs { swap(self, &mut rhs); self.negate(); }
				self.raw -= rhs.raw;
			}
		}
		impl Sub for BigInt { type Output = Self;
			fn sub(mut self, rhs: Self) -> Self { self -= rhs; self }
		}

		impl MulAssign for BigInt {
			fn mul_assign(&mut self, rhs: Self) {
				self.raw *= rhs.raw; self.minus ^= rhs.minus;
			}
		}
		impl Mul for BigInt { type Output = Self;
			fn mul(mut self, rhs: Self) -> Self { self *= rhs; self }
		}

		impl DivAssign for BigInt {
			fn div_assign(&mut self, rhs: Self) {
				self.raw /= rhs.raw; self.minus ^= rhs.minus;
			}
		}
		impl Div for BigInt { type Output = Self;
			fn div(mut self, rhs: Self) -> Self { self /= rhs; self }
		}

		impl BigInt {
			pub fn negate(&mut self) {
				if !self.is_zero() { self.minus = !self.minus; }
			}
			pub fn square(&mut self) {
				self.minus = false; self.raw.square();
			}

			pub fn divmod_euclid(self, rhs: Self) -> (Self, UbigInt) {
				let mut q = self.clone(); q /= rhs.clone();
				let mut rem = self-q.clone()*rhs.clone();
				if rem.is_neg() {
					if rhs.is_neg() { q += 1.into(); }
					else { q -= 1.into(); }
					rem += rhs.abs();
				}
				(q, rem.raw)
			}

			pub fn divmod(self, rhs: Self) -> (Self, Self) {
				let mut q = self.clone(); q /= rhs.clone();
				(q.clone(), self-q*rhs)
			}
		}
	}
} pub use bigint_mod::BigInt;
