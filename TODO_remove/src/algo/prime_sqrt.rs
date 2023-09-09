pub fn is_prime(n: u64) -> bool {
	for p in 2u64..=n {
		if p*p > n { return true; }
		if n%p == 0u64 { return false; }
	}
	false // n <= 1
}
