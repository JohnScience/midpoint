# What the crate should provide

The crate should provide functions that would allow to get midpoint with either no assumptions or with some
common assumptions about the arguments (think of safe and unsafe functions).

# Qualities

There are several ways to obtain the midpoint, each with some tradeoffs between generality, performance, and
algebraic properties.

## Rounding behavior

### Considered options

In the discussion "Average function for primitives"[^1], [Christopher Durham aka CAD97][CAD97] named five options for rounding that they consider reasonable:

* Round towards `a` (`lhs`);
* Round towards `b` (`rhs`);
* Round up (towards `MAX`);
* Round down (towards `MIN`);
* Round towards `0`;

and [user16251] proposed another dimension for the rounding behavior, *rounding towards even*, which as they claim
can be useful for fixed-point arithmetic.

### Systematization

Thus, to pinpoint the desired rounding behavior one should specify

1. whether it must be fundamentally absolute or relative:

   * Absolute
     * Round up (towards `MAX`);
     * Round down (towards `MIN`);
     * Round towards `0`;
   * Relative
     * Round towards `a` (`lhs`);
     * Round towards `b` (`rhs`);

2. And whether it should gravitate towards even values:

* Non-existent "even affinity" ([stipulative definition](https://www.ucfmapper.com/education/various-types-definitions/#:~:text=Stipulative%20definitions))

    It's irrelevant whether the final midpoint is even.
* Gravitational "even affinity"

    The final midpoint is guaranteed to be even.

#### Summary

Altogether, this amounts to astounding **10** different rounding behaviors **AND** even greater number of implementations if one accounts for implementations relying on frequently encountered assumptions.

The main potential negative impact of this combinatorial explosion is the codebloat that may arise in the codebase of the users of the crate. In order to prevent the codebloat, the library must provide its users with a straightforward way to use only the functions that they need and warn the users against using too high variety of them.

### Comments

[CAD97] gave the following opinion:

> Personally, I think only the first \["Round towards `a` (`lhs`)"\] or the fifth \["Round towards `0`"\] options are really in the running to be picked . The first isn't commutative, but it has predictable and consistent behavior for ++, +-, -+, and --: bias towards the first argument. The fifth leans on the fact that integer division truncates toward zero for familiarity, and gains commutativity, but loses the consistency that midpoint(x, y) == -midpoint(-x, -y).
>
> It's a trade-off. If always rounding toward zero can be done branchless and rounding toward a can't, that'd make me more likely to support rounding toward zero, but I still think rounding toward a is more useful. (Plus, if it inlines, you can just sort a and b on input to get the the rounding you want at no cost.)

## Parallelism

All implementations are meant to be used in single-threaded environment.

# Implementations

## Integers

### Naïve implementation

```rust
pub /*unsafe*/ fn $fn_name(a: &$t, b: &$t) -> $t {
    (a+b)/2
}
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively.

Arguably, it is the most efficient implementation when the sum of $a$ and $b$ can be stored (= calculated without overflow) in the original type. For primitive integers, the computation roughly amounts to loading the values into registers, performing a single add followed by a right shift by 1. The exact assembly can be found on [godbo.lt](https://godbolt.org/z/7Mzjvoe9P), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

However, $a+b$ cannot be guaranteed to be computed without overflow.

### Implementation relying on primitive promotion

```rust
    pub fn $fn_name(a: &$t, b: &$t) -> $t {
        let a = *a as <$t as PrimitivePromotion>::PrimitivePromotion;
        let b = *b as <$t as PrimitivePromotion>::PrimitivePromotion;
        ((a+b)/2) as $t
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively, and `u8::PrimitivePromotion` is `u16`, `i64::PrimitivePromotion` is `i128`, while `PrimitivePromotion` trait is not implemented for `u128` and `i128`.

As opposed to naive implementation, the implementation relying on primitive promotion is not defined for `u128` and `i128` yet it works as intended even when the sum of arguments does not fit into the original type \$t. The reason for this is that the sum is computed using the primitive promotion of \$t, where the overflow cannot happen given the arguments fit in \$t. The exact assembly can be found on [godbo.lt](https://godbolt.org/z/75h45e1no), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

### Implementation via wrapping sum of of right-shifted arguments and the LSB-masked bitwise AND of the arguments

```rust
    pub fn $fn_name(a: &$t, b: &$t) -> $t {
        (a >> 1).wrapping_add(b >> 1).wrapping_add(a & b & 0x1)
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively. The implementation was provided by [Eli Dupree] and relies on [wrapping_add], and [LSB]-[masked][bitmask] (meaning with all non-[LSB] bits masked off) bitwise AND of the arguments ("a & b & 0x1").

As opposed to previous implementations, the implementation via wrapping sum of of right-shifted arguments and the LSB-masked bitwise AND of the arguments produces the desired result regardless of the choice of primitive integer type, including signed integer types and 128-bit bit integers. For primitive signed integers, right shift (">>") corresponds to `SAR` (as opposed to `SHR` for unsigned integers) processor instruction when the compilation target uses x86 instruction set or its extension. The exact assembly can be found on [godbo.lt](https://godbo.lt/z/d133cP7oY), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

### Implementation via unchecked sum of right-shifted arguments and the LSB-masked bitwise AND of the arguments

```rust
    pub fn $fn_name(a: &$t, b: &$t) -> $t {
        // wrapping_add restricts the implementation,
        // unchecked_add is feature-gated and unsafe but
        // better reflects the known invariant (that the
        // addition won't overflow)
        unsafe{
            // >> compiler emits shr and sar for unsigned
            // and signed types, respectively
            (a >> 1)
                // The absolute value of n >> 1 is less than
                // or equal to the result of real division of n
                // by 2. Therefore, the absolute value of the
                // sum of near-halves cannot exceed neither
                // $t::MIN nor $t::MAX 
                .unchecked_add(b >> 1)
                // If the least significant bit of n s.t.
                // n=a=b is set to 0, "a & b & 0x1" is 0 and
                // the sum of halves cannot exceed n.
                // Otherwise, the corresponding right
                // shift discards the least significant digit
                // and then |2*(n >> 1)|=|n|-1.
                // Since "a & b & 0x1" is at most 1,
                // the expression can neither overflow nor
                // underflow.
                .unchecked_add(a & b & 0x1)
        }
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively.

This implementation is nearly identical to that of [Eli Dupree], yet instead of [wrapping_add] this implementation utilizes feature-gated unsafe [unchecked_add]. As the comment explains, [wrapping_add] restricts the implementation of addition, unlike [unchecked_add]. While these implementations produce the same assembly for x86 instruction set, [the author] is convinced that [unchecked_add] is better because the overflow is impossible in this case (and the proof is provided). The exact assembly can be found on [godbo.lt](https://godbolt.org/z/5bx8M7G5h), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

### Implementation via naive midpoint diff

```rust
    ($fn_name:ident, $t:ty) => {
        pub /*unsafe*/ fn $fn_name(a: &$t, b: &$t) -> $t {
            let arg_diff = b-a;
            let midpoint_diff = arg_diff/2;
            a + midpoint_diff
        }
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively.

As per "P0811R3: Well-behaved interpolation for numbers and pointers" by S. Davis Herring from Los Alamos National Laboratory[^2], this implementation is "the standard alternative" and...

> works for signed integers with the same sign (even if b<a), but can overflow if they have different signs. The modular arithmetic of unsigned integers does not produce the value expected for b<a because the division inherent to midpoint is not [native there](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0999r0.pdf); it instead produces the value halfway between a and the smallest modular equivalent to b that is no smaller.

To be precise, the overflow happens when b-a > $t::MAX and underflow happens when b-a < $t::MIN. As opposed to the two previous implementations, this one performs fewer but more complex operations. In addition, as S. Davis Herring noticed, it has serious limitations that may or may not be justified. The exact assembly can be found on [godbo.lt](https://godbolt.org/z/o6fse7ha3), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

### Implementation via less naive midpoint diff

```rust
    pub /*unsafe*/ fn $fn_name(a: &$t, b: &$t) -> $t {
        let arg_diff = (b-a) as <$t as EquisizedPrimitiveSignedInt>::EquisizedPrimitiveSignedInt;
        let midpoint_diff = (arg_diff/2) as $t;
        a + midpoint_diff
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively, and `u8::EquisizedPrimitiveSignedInt` is `i8`, `i64::EquisizedPrimitiveSignedInt` is `i64`, and so on.

For signed integers, ths implementation is identical to the implementation via naive diff. For unsigned integers, however, it returns the midpoint even in cases when b<\a unless a-b > $t::EquisizedPrimitiveSignedInt::MAX or a-b < $t::EquisizedPrimitiveSignedInt::MIN. The exact assembly can be found on [godbo.lt](https://godbolt.org/z/6jvf7hz6b), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

### C++ 20 standard library implementation

```c++
    constexpr Integer midpoint(Integer a, Integer b) noexcept {
        using U = make_unsigned_t<Integer>;
        return a>b ? a-(U(a)-b)/2 : a+(U(b)-a)/2;
    }
```

which, if tersity was the priority, would be rewritten in Rust by [the author] as:

```rust
    ($fn_name:ident, $t:ty) => {
        pub /*const*/ fn $fn_name(a: &$t, b: &$t) -> $t {
            use EquisizedPrimitiveUnsignedInt as EPUI;
            let u = |n: &$t| *n as <$t as EPUI>::EquisizedPrimitiveUnsignedInt;

            if a > b { a - ((u(a)-u(b))/2) as $t } else { a + ((u(b)-u(a))/2) as $t }
        }
    }
```

where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively, and `u8::EquisizedPrimitiveUnsignedInt` is `u8`, `i64::EquisizedPrimitiveUnsignedInt` is `u64`, and so on.

However, tersity is not the uttermost quality and, while short, this implementation does not convey the ideas that the developers had in their mind when they wrote the code. So, [the author] will attempt to achieve the following: (1) explain what happens in the code and (2) optimize the code for human readability without sacrificing its performance. The baseline for future comparison will be on [godbo.lt](https://godbolt.org/z/65eaKqrWP), where one can also run [llvm-mca] on the assembly for the purpose of static performance analysis.

The partially unfolded and partially refined (rather swollen) code is 

```rust
    ($fn_name:ident, $t:ty) => {
        pub const fn $fn_name(a_ref: &$t, b_ref: &$t) -> $t {
            use EquisizedPrimitiveUnsignedInt as EPUI;
            type U = <$t as EPUI>::EquisizedPrimitiveUnsignedInt;

            // Without explicit dereferencing, the function
            // wouldn't be const-qualified. Hungarian-like notation
            // is a necessary evil.
            let a = *a_ref;
            let b = *b_ref;
            // Similarly to C++, according to Rust's reference
            // (https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast),
            // the bit patterns (https://en.wikipedia.org/wiki/Type_conversion#:~:text=bit%20pattern)
            // of the arguments get safely reinterpreted
            // (https://en.wikipedia.org/wiki/Type_conversion#:~:text=interpretation%20of%20the%20bit%20pattern)
            // at compile time as the values of the equally-sized
            // unsigned  int via cast both in the narrow
            // (and the broader) sense
            // (https://en.wikipedia.org/wiki/Type_conversion#:~:text=The%20word%20cast)
            let u_a = a as U;
            let u_b = b as U;
            debug_assert!( u_a == unsafe { core::mem::transmute::<$t,U>(a) });
            debug_assert!( u_b == unsafe { core::mem::transmute::<$t,U>(b) });

            // Type limit comparisons are deemed useless
            // by the compiler but they demonstrate
            // the known invariants to the programmer
            #[allow(unused_comparisons)]
            if a > b {
                let u_a_sub_u_b = u_a-u_b;
                // Since a > b, a-b >= 1
                debug_assert!(u_a_sub_u_b >= 1);
                debug_assert!(u_a_sub_u_b <= U::MAX);
                // Ideally, there must be a constructive theorem for the lower bound
                //
                // In this case, the theorem that 1 is the result for
                // every pair (u_a,u_b) of the image of [0..U::MAX - 1] under
                // n ↦ (n+1,n)
                debug_assert!(U::MAX-0 == U::MAX);
                debug_assert!(
                    u_a_sub_u_b != U::MAX || u_a == U::MAX && u_b == 0
                );
                let u_midpoint_diff_down = u_a_sub_u_b/2;
                debug_assert!(u_midpoint_diff_down >= 0);
                debug_assert!(u_midpoint_diff_down <= U::MAX/2);
                // Ideally, there must be a constructive theorem for the lower bound
                debug_assert!(
                    u_midpoint_diff_down != U::MAX/2 || u_a == U::MAX && u_b == 0
                );
                let midpoint_diff_down = u_midpoint_diff_down as $t;
                // The assert below is impossible but const_format crate
                // allows to perform comparison of static core::str's
                //
                // debug_assert!(
                //     core::any::TypeId::of::<$t>() == core::any::TypeId::of::<U>() || {
                //         (U::MAX/2).to_string() == <$t>::MAX.to_string()
                //     }
                // );
                debug_assert!(
                    a-midpoint_diff_down == a-(((u_a - u_b)/2) as $t) 
                );
                a - midpoint_diff_down
            } else {
                let midpoint_diff_up = ((u_b-u_a)/2) as $t;
                a + midpoint_diff_up
            }
        }
    }
```
where \$fn_name and \$t are [identifier and type designator](https://doc.rust-lang.org/rust-by-example/macros/designators.html), respectively, and `u8::EquisizedPrimitiveUnsignedInt` is `u8`, `i64::EquisizedPrimitiveUnsignedInt` is `u64`, and so on.

While more informative, however, this implementation does not communicate clearly the inherent near-symmetry of the algorithm. In this situtation, the best solution for uncluttering the code would be to supply external documentation for the implementation (and, potentially, develop static analysis tools for performing [bounds checking](https://en.wikipedia.org/wiki/Bounds_checking) on demand). However, [the author] is unsure what to do with communicating the near-symmetric nature of the algorithm.

[^1]: https://internals.rust-lang.org/t/average-function-for-primitives/14040
[^2]: http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p0811r3.html

[CAD97]: https://internals.rust-lang.org/u/CAD97
[user16251]: https://internals.rust-lang.org/u/user16251
[Eli Dupree]: https://internals.rust-lang.org/u/elidupree
[the author]: https://github.com/JohnScience

[llvm-mca]: https://www.youtube.com/watch?v=Ku2D8bjEGXk

[wrapping_add]: https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add
[unchecked_add]: https://doc.rust-lang.org/std/primitive.u32.html#method.unchecked_add
[LSB]: https://en.wikipedia.org/wiki/Bit_numbering#:~:text=In%20computing%2C%20the%20least%20significant,1s%20place%20of%20the%20integer.&text=The%20LSB%20is%20sometimes%20referred,digits%20further%20to%20the%20right.
[bitmask]: https://en.wikipedia.org/wiki/Mask_(computing)