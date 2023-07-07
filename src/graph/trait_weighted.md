# Weighted Graph Trait

A `WGraph` trait should implement the following:

`type Neighbor<'a>: Iterator<Item = &'a (usize, u64)> where Self: 'a`

- 💬 The iterator type for a sequence of vertices and edge-costs, representing the adjacent vertices to some given vertex.

`fn size(&self) -> usize`

- 💬 The number of vertices.

`fn neighbor<'a>(&'a self, v: usize) -> Self::Neighbor<'a>`

- 💬 The iterator of all vertices adjacent to \\( v \\).

## Example

TODO

## Code
```rust,noplayground
trait WGraph {
    type Neighbor<'a>: Iterator<Item = &'a (usize, u64)> where Self: 'a;
    fn size(&self) -> usize;
    fn neighbor<'a>(&'a self, v: usize) -> Self::Neighbor<'a>;
}
```