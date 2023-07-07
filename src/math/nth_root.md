# Nth Root

*Who needs binary search when you can just power it by \\( \frac{1}{f} \\) and correct errors?*

`fn nth_root(n: u64, f: u64) -> Option<u64>`
- 💬 If \\( n = x^f \\) for some non-negative integer \\( x \\), returns \\( Some(x) \\). Otherwise, returns \\( None \\).

TODO: this code isn't thoroughly tested. It's most likely correct, but maybe `+-3` is too wide and `+-1` might actually be enough. I also don't know how fast it is compared to binary search.

## Code
```rust
fn nth_root(n: u64, f: u64) -> Option<u64> {
    let guess: u64 = f64::powf(n as f64, 1.0 / f as f64) as u64;
    for g in (guess.saturating_sub(3))..guess+3 {
        if g.saturating_pow(f as u32) == n { return Some(g); }
    }
    None
}
```