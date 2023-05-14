# Barrett Reduction

## `Barrett`

- 💬 A utility for fixed-mod where the modulo base is not known in compile time.
- ℹ️ It is much faster than a naive `%`.

`fn new(n: u64) -> Self`

- 🏗️ Creates a new fixed-mod utility for modulo \\( n \\).
- ⚠️ Panics from division by 0 if \\( n = 0 \\).

`fn reduce(&self, x: u64) -> u64`

- 💬 Returns \\( x \\) modulo \\( n \\).

`fn modpow(&self, mut x: u64, mut k: u64) -> u64`

- 💬 Returns \\( x^k \\) modulo \\( n \\).

## Example

```rust
#pub struct Barrett { n: u64, m: u128 }
#impl Barrett {
#    pub fn new(n: u64) -> Self {
#        Barrett { n: n, m: (1u128 << 64) / n as u128 }
#    }
#
#    // x mod n
#    pub fn reduce(&self, x: u64) -> u64 {
#        let q = ((self.m * (x as u128)) >> 64) as u64;
#        let r = x - q * self.n;
#        if r >= self.n { r - self.n } else { r }
#    }
#}
#
fn mod_factorial(n: u64, m: u64) -> u64 {
    let br = Barrett::new(m);
    let mut f: u64 = 1;
    for i in 1..=n {
        f = br.reduce(f * i);
    }
    f
}

fn main() {
    println!("{}", mod_factorial(10, 1_000_000)); // 628800
    println!("{}", mod_factorial(100_000_000, 1_000_000_007)); // 927880474
}
```

## Code

```rust,noplayground
pub struct Barrett { n: u64, m: u128 }
impl Barrett {
    pub fn new(n: u64) -> Self {
        Barrett { n: n, m: (1u128 << 64) / n as u128 }
    }

    // x mod n
    pub fn reduce(&self, x: u64) -> u64 {
        let q = ((self.m * (x as u128)) >> 64) as u64;
        let r = x - q * self.n;
        if r >= self.n { r - self.n } else { r }
    }

    // x^k mod n
    pub fn modpow(&self, mut x: u64, mut k: u64) -> u64 {
        let mut ans = 1u64;
        while k != 0 {
            if (k&1) != 0 { ans = self.reduce(ans * x); }
            k>>= 1; x = self.reduce(x * x);
        }
        ans
    }
}
```