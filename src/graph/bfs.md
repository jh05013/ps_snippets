# BFS
`fn bfs<G: Graph>(graph: &G, start: usize) -> Vec<Option<u32>>`
- 💬 Returns a vector \\( D \\) of size \\( |V(G)| \\).
- 💬 If a vertex \\( v \\) is unreachable from \\( start \\), then \\( D[v] = None \\).
- 💬 Otherwise, \\( D[v] = Some(d) \\) where \\( d \\) is the distance from \\( start \\) to \\( v \\).
- 🕒 \\( |V(G)| \\).
- ⚠️ Panics with OoB if \\( start \geq |V(G)| \\).

## Example
```rust,noplayground
fn main() {
    let mut g = AdjGraph::new(5, false);
    g.connect(0, 2);            //           0
    g.connect(2, 1);            //          / \     4
    g.connect(2, 3);            //         *   *
    g.connect(0, 3);            //  1 *-- 2 --* 3
    print!("{:?}", bfs(&g, 0)); // [Some(0), Some(2), Some(1), Some(1), None]
}
```

## Code
```rust,noplayground
fn bfs<G: Graph>(graph: &G, start: usize) -> Vec<Option<u32>> {
    let mut dist: Vec<Option<u32>> = vec![None; graph.size()];
    dist[start] = Some(0);
    let mut q = VecDeque::<usize>::new();
    q.push_back(start);
    while !q.is_empty() {
        let a: usize = q.pop_front().unwrap();
        for b in graph.neighbor(a) {
            if let Some(_) = dist[*b] { continue; }
            dist[*b] = Some(dist[a].unwrap() + 1);
            q.push_back(*b);
        }
    }
    dist
}
```