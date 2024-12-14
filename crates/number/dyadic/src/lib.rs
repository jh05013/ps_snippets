//! Explanation

use std::{iter::Sum, ops::*};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dyadic {
    numer: i64,
    denom_log: u32,
}

impl From<i64> for Dyadic {
    fn from(value: i64) -> Self {
        Self {
            numer: value,
            denom_log: 0,
        }
    }
}

impl std::fmt::Display for Dyadic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / 2^{}", self.numer, self.denom_log)
    }
}

impl PartialOrd for Dyadic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Dyadic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (a, b) = self.commonize_denom(other);
        a.numer.cmp(&b.numer)
    }
}

impl Dyadic {
    #[must_use]
    pub fn new(numer: i64, denom_log: u32) -> Self {
        // invariant: numerator is odd unless denom_log is 0.
        let d = numer.trailing_zeros().min(denom_log);
        Self {
            numer: numer >> d,
            denom_log: denom_log - d,
        }
    }

    #[must_use]
    pub fn ceil(&self) -> i64 {
        let add = i64::from(self.denom_log != 0);
        (self.numer >> self.denom_log) + add
    }

    fn commonize_denom(&self, other: &Self) -> (Self, Self) {
        let denom_log = self.denom_log.max(other.denom_log);
        let self_numer = self.numer << (denom_log - self.denom_log);
        let other_numer = other.numer << (denom_log - other.denom_log);
        (
            Self {
                numer: self_numer,
                denom_log,
            },
            Self {
                numer: other_numer,
                denom_log,
            },
        )
    }
}

impl Neg for Dyadic {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            numer: -self.numer,
            denom_log: self.denom_log,
        }
    }
}

impl Add for Dyadic {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (a, b) = self.commonize_denom(&rhs);
        Self::new(a.numer + b.numer, a.denom_log)
    }
}

impl Sub for Dyadic {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (a, b) = self.commonize_denom(&rhs);
        Self::new(a.numer - b.numer, a.denom_log)
    }
}

impl Mul<Self> for Dyadic {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.numer * rhs.numer, self.denom_log + rhs.denom_log)
    }
}

impl Sum for Dyadic {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

#[must_use]
pub fn value_of_game(left: Option<Dyadic>, right: Option<Dyadic>) -> Option<Dyadic> {
    let zero = Dyadic::default();

    let Some(right) = right else {
        let Some(left) = left else {
            return Some(Dyadic::default());
        };
        if left < zero {
            return Some(zero);
        }
        let numer = (left.numer >> left.denom_log) + 1;
        return Some(Dyadic {
            numer,
            denom_log: 0,
        });
    };
    let Some(left) = left else {
        return Some(-(value_of_game(Some(-right), None).unwrap()));
    };

    if left >= right {
        return None;
    }
    if left < zero {
        if right > zero {
            return Some(zero);
        }
        return Some(-(value_of_game(Some(-right), Some(-left)).unwrap()));
    }

    // left > 0
    // require: left/2^leftdl < x/2^newdl
    for new_denom_log in 0.. {
        let new_numer = if new_denom_log >= left.denom_log {
            // left * 2^(newdl - leftdl) < x
            (left.numer << (new_denom_log - left.denom_log)) + 1
        } else {
            // left < x * 2^(leftdl - newdl)
            // note that left.numer is odd
            (left.numer >> (left.denom_log - new_denom_log)) + 1
        };

        let ans = Dyadic {
            numer: new_numer,
            denom_log: new_denom_log,
        };
        assert!(left < ans);
        if ans < right {
            return Some(ans);
        }
    }
    unreachable!()
}
