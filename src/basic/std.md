# std Goodness

Don't reinvent the wheel! Here are std stuffs I find useful in competitive programming/problem solving. Nightly features are not included.

## Integers

[i32 docs](https://doc.rust-lang.org/std/primitive.i32.html), [u32 docs](https://doc.rust-lang.org/std/primitive.i32.html)

| Methods | Purposes |
| --- | --- |
| `abs`, `ilog...`, `div_euclid`, `rem_euclid`, `pow`, `signum` | Math |
| `count_...`, `leading_...`, `trailing_...`, `next_power_of_two` | Bit operations |
| `abs_diff`, `checked_...`, `saturating_...` | Avoiding overflows |

## Floats

[f64 docs](https://doc.rust-lang.org/std/primitive.f64.html)

| Methods | Purposes |
| --- | --- |
| `abs`, `sin`, ..., `asin`, ..., `sinh`, ..., `sin_cos`, `exp...`, `ln...`, `log...`, `pow...`, `sqrt`, `signum` | Math |
| `clamp` | Restricting the number inside a range |
| `floor`, `ceil`, `fract`, `round`, `trunc` | Rounding |
| `max`, `min`, `total_cmp` | Sidestepping NaN |
| `hypot`, `to_degrees`, `to_radians` | Float geometry |

Take a look at [consts](https://doc.rust-lang.org/std/f64/consts/index.html) too, especially `E`, `PI`, and `SQRT_2`.

## Vec and Slice

[Vec docs](https://doc.rust-lang.org/std/vec/struct.Vec.html)

| Methods | Purposes |
| --- | --- |
| `new`, `clear`, `is_empty`, `len`, `push`, `pop` | Every language has these, right? |
| `contains`, `starts_with`, `ends_with` | Contents checking |
| `insert`, `remove`, `swap` | Middle index manipulation |
| `sort...`, `is_sorted...`, `reverse`, `rotate...`, `select_nth_unstable...` | Order manipulation |
| `fill`, `append`, `drain`, `splice`, `split_off`, `resize`, `truncate` | Subrange manipulation |
| `get...`, `last` | Indexing without panicking |
| `binary_search...`, `partition_point` | Binary search |
| `retain...` | Filter |
| `concat`, `join` | Flattening |
| `dedup...` | Retaining unique elements |
| `repeat` | Multiplication |
| `iter...`, `(r)chunks...`, `(r)split...`, `windows` | Iteration |

Check out the `vec!` macro too.

## String and str

[String docs](https://doc.rust-lang.org/std/string/struct.String.html)

| Methods | Purposes |
| --- | --- |
| `new`, `clear`, `is_empty`, `len`, `push`, `pop` | Every language has these, right? |
| `as_bytes` | Lookup |
| `eq_ignore_ascii_case`, `make_ascii_...case`, `to_...case` | ASCII |
| `insert...` | Middle index manipulation |
| `push_str`, `replace(n)`, `replace_range`, `split_off`, `truncate` | Substring manipulation |
| `contains`, `(r)find`, `(r)matches`, `(r)match_indices`, `starts_with`, `ends_with` | Pattern matching |
| `retain` | Filter |
| `parse` | Parsing |
| `repeat` | Multiplication |
| `chars`, `(r)split...` | Iteration |

## collections
