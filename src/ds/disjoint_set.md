# Disjoint Set

## `DisjointSet`

- 💬 A data structure to keep track of partitions that gradually merge with each other.

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
# #[derive(Debug, Eq, PartialEq, Clone)]
#pub struct DisjointSet { par: Vec::<usize> }
#impl DisjointSet {
#    pub fn new(n: usize) -> Self { DisjointSet { par: (0..n).collect() } }
#    pub fn len(&self) -> usize { self.par.len() }
#    pub fn union(&mut self, a: usize, b: usize) {
#        let ar = self.find(a);
#        self.par[ar] = self.find(b);
#    }
#    pub fn find(&mut self, i: usize) -> usize {
#        if self.par[i] == i { return i; }
#        self.par[i] = self.find(self.par[i]);
#        self.par[i]
#    }
#}
#
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
    pub fn find(&mut self, i: usize) -> usize {
        if self.par[i] == i { return i; }
        self.par[i] = self.find(self.par[i]);
        self.par[i]
    }
}
```