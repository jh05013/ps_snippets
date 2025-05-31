use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub const LOG_RADIX: usize = 6;
pub const RADIX: u32 = 10u32.pow(LOG_RADIX as u32);

#[derive(Clone, Debug, Default)]
pub struct UBigInt(Vec<u32>);

impl From<u32> for UBigInt {
	fn from(value: u32) -> Self {
		if value == 0 { Self::default() }
		else { Self(vec![value]) }
	}
}

impl FromStr for UBigInt {
	type Err = char;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut inner = vec![];
		for chunk in s.as_bytes().rchunks(LOG_RADIX) {
			let mut num = 0u32;
			for c in chunk {
				if !c.is_ascii_digit() {
					return Err(char::from(*c));
				}
				num = num * 10 + ((c - b'0') as u32);
			}
			inner.push(num);
		}
		Ok(UBigInt(inner))
	}
}

impl Display for UBigInt {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut is_first = true;
		for x in self.0.iter().rev() {
			if is_first {
				write!(f, "{x}")?;
			}
			else {
				write!(f, "{x:00$}", LOG_RADIX)?;
				is_first = false;
			}
		}
		Ok(())
	}
}

impl std::ops::Add<&UBigInt> for UBigInt {
	type Output = Self;

	fn add(mut self, rhs: &UBigInt) -> Self::Output {
		let mut carry = 0u32;
		for (i, y) in rhs.0.iter().enumerate() {
			let y = *y + carry;
			if let Some(x) = self.0.get_mut(i) {
				*x += y;
				if *x >= RADIX {
					*x -= RADIX;
					carry = 1;
				}
				else {
					carry = 0;
				}
			}
			else if y == RADIX {
				self.0.push(0);
			}
			else {
				self.0.push(y);
				carry = 0;
			}
		}
		self
	}
}

impl std::ops::Mul<u32> for UBigInt {
	type Output = Self;

	fn mul(mut self, rhs: u32) -> Self::Output {
		let mut carry = 0u64;
		for x in &mut self.0 {
			let x_new = (*x as u64) * (rhs as u64) + carry;
			*x = (x_new % (RADIX as u64)) as u32;
			carry = x_new / (RADIX as u64);
		}
		if carry != 0 {
			self.0.push(carry as u32);
		}
		self
	}
}