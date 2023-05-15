# Sqrt-time Primality

A simple \\( O(\sqrt{n}) \\)-time primality test.

`fn is_prime(n: u64) -> bool`
- 💬 Returns `true` iff \\( n \\) is a prime.
- 🕒 \\( O(\sqrt{n}) \\).

## Example
```rust
#fn is_prime(n: u64) -> bool {
#    for p in 2u64..=n {
#        if p*p > n { return true; }
#        if n%p == 0u64 { return false; }
#    }
#    false // n <= 1
#}
#
#fn main() {
for n in 0..100 {
    if is_prime(n) {
        print!("{} ", n); // 2 3 5 ... 97
    }
}
#}
```

## Code
```rust,noplayground
fn is_prime(n: u64) -> bool {
    for p in 2u64..=n {
        if p*p > n { return true; }
        if n%p == 0u64 { return false; }
    }
    false // n <= 1
}
```