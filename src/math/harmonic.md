# Harmonic Lemma

It can be proven that for a non-negative integer \\( N \\), the expression \\( \lfloor \frac{N}{x} \rfloor \\) for integer \\( x \\) can have \\( O(\sqrt{N}) \\) different values. The same is true for \\( \lceil \frac{N}{x} \rceil \\). This page describes an iterator for those values.

The implementation looks convoluted, but the purpose is to make it as fast as possible by doing as few divisions as possible (once per item).

No generic is used since this can't really be used outside 64-bit or smaller integers.

## `DivFloors`

`fn new(n: u64) -> Self`

- 🏗️ Creates a new floor division iterator \\( n \\).

`impl Iterator for DivFloors`

- 💬 Each item \\( (v, l, r) \\) means that \\( \lfloor \frac{N}{x} \rfloor = x \\) for \\( x \\) in \\( [l, r] \\).
- 💬 The items are returned in the decreasing order of \\( v \\) until \\( v = 1 \\).
- 🕒 There are \\( O(\sqrt{n}) \\) items, each computed in \\( O(1) \\).

## `DivCeils`

TODO implement this

## Example
```rust
# struct DivFloors { n: u64, x: u64, last: u64 }
# impl DivFloors {
#     fn new(n: u64) -> Self { Self { n, x: 1, last: 0 } }
# }
# // (v,l,r) where n/x = v for x in [l, r]
# impl Iterator for DivFloors {
#     type Item = (u64, u64, u64);
#     fn next(&mut self) -> Option<Self::Item> {
#         let (n, x) = (self.n, self.x);
#         if self.last == 0 {
#             let item = Some((n/x, x, x));
#             self.x += 1;
#             if self.x.pow(2) > n { self.last = self.x - 1; self.x = n/self.x; }
#             return item;
#         }
#         if x == 0 { return None; }
#         let new_last = n/x;
#         let item = Some((x, self.last + 1, new_last));
#         self.last = new_last;
#         self.x -= 1;
#         item
#     }
# }
# 
# fn main() {
/*
x    1 2 3 4 5 6 7 8 9 10 11 12 13
------------------------------------
N/x 10 5 3 2 2 1 1 1 1  1  0  0  0
*/
for item in DivFloors::new(10) {
    print!("{:?} ", item);
    // (10, 1, 1) (5, 2, 2) (3, 3, 3) (2, 4, 5) (1, 6, 10)
}
# }
```

## Code
```rust,noplayground
struct DivFloors { n: u64, x: u64, last: u64 }
impl DivFloors {
    fn new(n: u64) -> Self { Self { n, x: 1, last: 0 } }
}
// (v,l,r) where n/x = v for x in [l, r]
impl Iterator for DivFloors {
    type Item = (u64, u64, u64);
    fn next(&mut self) -> Option<Self::Item> {
        let (n, x) = (self.n, self.x);
        if self.last == 0 {
            let item = Some((n/x, x, x));
            self.x += 1;
            if self.x.pow(2) > n { self.last = self.x - 1; self.x = n/self.x; }
            return item;
        }
        if x == 0 { return None; }
        let new_last = n/x;
        let item = Some((x, self.last + 1, new_last));
        self.last = new_last;
        self.x -= 1;
        item
    }
}
```

# Practice Problems
- [LC Enumerate Quotients](https://judge.yosupo.jp/problem/enumerate_quotients) Floor, 1e12, 108 ms
- [BOJ 26056 수열의 합 2](https://www.acmicpc.net/problem/26056) Floor, 1e14, twice in 436 ms
- [BOJ 15897 잘못 구현한 에라토스테네스의 체](https://www.acmicpc.net/problem/15897) Ceil, 1e9, TODO
