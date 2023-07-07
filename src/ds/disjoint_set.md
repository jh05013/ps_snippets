# Disjoint Set

Disjoint set with no extra information.

The optimization technique employed is [path halving](https://en.wikipedia.org/wiki/Disjoint-set_data_structure#Finding_set_representatives). It seems to be faster than path compression in practice.

## `DisjointSet`

`fn new(n: usize) -> Self`

- 🏗️ Creates a disjoint set structure representing \\( \\{ \\{0\\}, \\{1\\}, \\cdots, \\{n-1\\} \\} \\).

`fn len(&self) -> usize`

- 💬 Returns \\( n \\) .

`fn union(&mut self, a: usize, b: usize)`

- 💬 Merges the set containing \\( a \\) with the set containing \\( b \\).
- 🕒 Amortized \\( O(\log n) \\).
- ℹ️ If \\( a \\) and \\( b \\) are already in the same set, nothing happens.
- ⚠️ Panics with OoB if \\( a \geq n \vee b \geq n \\).

`fn find(&mut self, i: usize) -> usize`

- 💬 Returns one of the numbers in the set containing \\( i \\).
- 💬 `find(a) == find(b)` iff \\( a \\) and \\( b \\) are in the same set.
- 🕒 Amortized \\( O(\log n) \\).
- ⚠️ Panics with OoB if \\( a \geq n \vee b \geq n \\).

## Example

```rust
fn main() {
    let mut ds = DisjointSet::new(5);
    println!("{}", ds.len());                 // 5
    println!("{}", ds.find(0) == ds.find(2)); // false
    ds.union(0, 1);
    ds.union(2, 1);
    println!("{}", ds.find(0) == ds.find(2)); // true
}
```

## Code

```rust,noplayground
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DisjointSet { par: Vec::<usize> }
impl DisjointSet {
    pub fn new(n: usize) -> Self { DisjointSet { par: (0..n).collect() } }
    pub fn len(&self) -> usize { self.par.len() }
    pub fn union(&mut self, a: usize, b: usize) {
        let ar = self.find(a);
        self.par[ar] = self.find(b);
    }
    pub fn find(&mut self, mut i: usize) -> usize {
        while self.par[i] != i {
            self.par[i] = self.par[self.par[i]];
            i = self.par[i];
        }
        i
    }
}
```

# Practice Problems
- [LC Unionfind](https://judge.yosupo.jp/problem/unionfind) 200k elements, 200k queries, 23 ms
- [BOJ 1717 집합의 표현](https://www.acmicpc.net/problem/1717) 1M elements, 100k queries, 28 ms
- [BOJ 20040 사이클 게임](https://www.acmicpc.net/problem/20040) 500k elements, 1M queries, 40 ms

(BOJ 20040 looks like 2M queries, but it's actually 1M queries since the answer cannot be over 500k)
