# Graph Trait

A `Graph` trait should implement the following:

`type Neighbor<'a>: Iterator<Item = &'a usize> where Self: 'a`

- 💬 The iterator type for a sequence of vertices, representing the adjacent vertices to some given vertex.

`fn size(&self) -> usize`

- 💬 The number of vertices.

`fn neighbor<'a>(&'a self, v: usize) -> Self::Neighbor<'a>`

- 💬 The iterator of all vertices adjacent to \\( v \\).

## Example

See [Adjacency List](adj.md).

## Code
```rust,noplayground
trait Graph {
    type Neighbor<'a>: Iterator<Item = &'a usize> where Self: 'a;
    fn size(&self) -> usize;
    fn neighbor<'a>(&'a self, v: usize) -> Self::Neighbor<'a>;
}
```