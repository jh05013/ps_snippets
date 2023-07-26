pub trait Gcd { fn gcd(self, other: Self) -> Self; }
pub trait Lcm { fn lcm(self, other: Self) -> Self; }

impl<T> Gcd for T where
T: PartialEq + Clone + Default + std::ops::Rem<Output = T> {
	fn gcd(self, b: T) -> T {
		if b == T::default() { self } else { b.clone().gcd(self%b) }
	}
}
impl<T> Lcm for T where
T: Clone + Gcd + std::ops::Mul<Output = T> + std::ops::Div<Output = T> {
	fn lcm(self, b: T) -> T { self.clone()*b.clone() / self.gcd(b) }
}
