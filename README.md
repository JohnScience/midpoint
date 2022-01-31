# Midpoint two-place function

Two-place midpoint function is the function returning an average of two values, such as values of signed or unsigned integer types, floating point types, or pointer types.

This library provides several implementations of two-place midpoint function \[currently, only for primitive integers\] with different properties (performance, generality, and rounding behavior) whereas the [GitHub repo of the lib](https://github.com/JohnScience/midpoint) offers the design document, tests, runnable benchmarks, and pre-generated [criterion.rs performance reports](https://github.com/bheisler/criterion.rs).

# Example

## Cargo.toml

```toml
[dependencies]
midpoint = { version = "0.1.2" }

# Read more about features here: https://doc.rust-lang.org/cargo/reference/features.html#dependency-features
[features]
all = ["const_trait_impl", "unchecked_math", "const_inherent_unchecked_arith"]
const_trait_impl = ["midpoint/const_trait_impl"]
unchecked_math = ["midpoint/unchecked_math"]
const_inherent_unchecked_arith = ["midpoint/const_inherent_unchecked_arith"]
```

## src/main.rs

```rust
use midpoint::MidpointViaPrimitivePromotionExt;

// With features = ["all"] or 
// features = ["const_trait_impl", ...] the call can be
// performed in constant context, such as const fn
let result: i32 = (-7).midpoint_via_primitive_promotion(&-2);
assert_eq!(result, -4);
```

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>