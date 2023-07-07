# Cheatsheet

## HashMap

Unlike C++, hash maps in Rust are faster than tree maps and [are DoS-resilient](https://doc.rust-lang.org/std/collections/struct.HashMap.html). Feel free to use them!

Self-explanatory basic APIs:
- `new`, `clear`, `is_empty`, `len`
- `contains_key`, `insert`, `remove`
- `iter`, `iter_mut`, `keys`, `values`, `values_mut`

Lookup with key `k`, but if there's no entry, use a default value `d`:

```rust
map.get(&k).unwrap_or(&d);
```

Add `inc` to the value for the key `k`, assuming initially 0 if there's no entry:

```rust
map.entry(k).and_modify(|v| *v+= inc).or_insert(inc);
```

**Practice problems**
- [LC Associative Array](https://judge.yosupo.jp/problem/associative_array)
