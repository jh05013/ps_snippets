//! Exponentiation by squaring.
//!
//! TODO add examples

/// Returns `a^b modulo m`.
pub fn modpow_generic<T: Clone>(a: T, mut b: u64, initial: T, multiply: impl Fn(&mut T, &T)) -> T {
    let mut ans = initial;
    let mut mul = a;
    while b != 0 {
        if b & 1 == 1 {
            multiply(&mut ans, &mul);
        }
        let mul_self = mul.clone();
        multiply(&mut mul, &mul_self);
        b >>= 1;
    }
    ans
}

/// Returns `a^b modulo m`.
/// ```
/// # use modpow::modpow_u64;
/// assert_eq!(modpow_u64(2, 4, 10), 6); // 2^4 = 16
/// assert_eq!(modpow_u64(3, 0, 10), 1); // 3^0 = 1
/// ```
pub fn modpow_u64(a: u64, b: u64, md: u64) -> u64 {
    modpow_generic(a, b, 1 % md, |x, a| {
        *x = *x * *a % md;
    })
}
