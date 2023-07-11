# Static Search

For a fixed array, this data structure supports a range search for specific values.

## `StaticSearch`

Trait bound: `T: Copy + Eq + Hash`

`fn new(vals: &[T]) -> Self`

- 🏗️ Creates a static range search structure for the array \\( vals \\).
- 🕒 \\( O( | vals | ) \\).

`fn last_until(&self, v: T, idx: usize) -> Option<usize>`

- 💬 If there is an index \\( i \leq idx \\) s.t \\( vals[i] = v \\), returns \\( Some(i_{max}) \\) where \\( i_{max} \\) is the largest such \\( i \\).
- 💬 Otherwise, returns \\( None \\).
- 🕒 \\( O( \log | vals | ) \\).
- ℹ️ \\( idx \\) *can* be out of bounds, in which case the largest valid \\( i \\) will be searched. Should I prevent this?

`fn first_from(&self, v: T, idx: usize) -> Option<usize>`

- 💬 If there is an index \\( i \geq idx \\) s.t \\( vals[i] = v \\), returns \\( Some(i_{min}) \\) where \\( i_{min} \\) is the smallest such \\( i \\).
- 💬 Otherwise, returns \\( None \\).
- 🕒 \\( O( \log | vals | ) \\).
- ℹ️ \\( idx \\) *can* be out of bounds, in which case \\( None \\) will be returned. Should I prevent this?

`fn count_range(&self, v: T, l: usize, r: usize) -> usize`

- 💬 Returns the number of \\( i \\) s.t \\( l \leq i \leq r \\) and \\( vals[i] = v \\).
- 💬 If \\( l > r \\), returns \\( 0 \\).
- 🕒 \\( O( \log | vals | ) \\).
- ℹ️ \\( l \\) and \\( r \\) *can* be out of bounds. Should I prevent this?

## Example

```rust,noplayground
let sts = StaticSearch::new(&[1, 3, 3, 1, 2, 3]);
println!("{:?}", sts.last_until(1, 2)); // Some(0)
println!("{:?}", sts.last_until(1, 3)); // Some(3)
println!("{:?}", sts.last_until(3, 1)); // Some(1)
println!("{:?}", sts.last_until(99, 5)); // None
println!("{:?}", sts.first_from(1, 2)); // Some(3)
println!("{:?}", sts.first_from(1, 4)); // None
println!("{}", sts.count_range(3, 2, 5)); // 2
println!("{}", sts.count_range(3, 5, 2)); // 0
println!("{}", sts.count_range(99, 2, 5)); // 0
```

## Code

```rust
mod static_search {
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct StaticSearch<T> {
        indices: HashMap<T, Vec<usize>>
    }
    
    #[allow(dead_code)]
    impl<T: Copy + Eq + std::hash::Hash> StaticSearch<T> {
        pub fn new(vals: &[T]) -> Self {
            let mut indices = HashMap::<T, Vec<usize>>::new();
            for (i, v) in vals.iter().enumerate() {
                indices.entry(*v).and_modify(|inds| inds.push(i)).or_insert(vec![i]);
            }
            Self { indices }
        }

        pub fn last_until(&self, v: T, idx: usize) -> Option<usize> {
            if let Some(inds) = self.indices.get(&v) {
                let i = inds.partition_point(|i| *i <= idx);
                if i == 0 { None } else { Some(inds[i-1]) }
            }
            else { None }
        }

        pub fn first_from(&self, v: T, idx: usize) -> Option<usize> {
            if let Some(inds) = self.indices.get(&v) {
                let i = inds.partition_point(|i| *i < idx);
                inds.get(i).copied()
            }
            else { None }
        }

        pub fn count_range(&self, v: T, l: usize, r: usize) -> usize {
            if l > r { 0 }
            else if let Some(inds) = self.indices.get(&v) {
                let li = inds.partition_point(|i| *i < l);
                let ri = inds.partition_point(|i| *i <= r);
                ri - li
            }
            else { 0 }
        }
    }
} use static_search::*;
#
#fn main() {
#    let sts = StaticSearch::new(&[1, 3, 3, 1, 2, 3]);
#    println!("{:?}", sts.last_until(1, 2));
#    println!("{:?}", sts.last_until(1, 3));
#    println!("{:?}", sts.last_until(3, 1));
#    println!("{:?}", sts.last_until(99, 5));
#    println!("{:?}", sts.first_from(1, 2));
#    println!("{:?}", sts.first_from(1, 4));
#    println!("{}", sts.count_range(3, 2, 5));
#    println!("{}", sts.count_range(3, 5, 2));
#    println!("{}", sts.count_range(99, 2, 5));
#}
```

# Practice Problems
- [LC Static Range Frequency](https://judge.yosupo.jp/problem/static_range_frequency) `count_range`, 500k elements, 500k queries, 270 ms
