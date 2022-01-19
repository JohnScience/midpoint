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

## Parallelism

All implementations are meant to be used in single-threaded environment.

# Implementations

## Integers

### Naïve implementation

$$(a+b)/2$$

Arguably, it is the most efficient implementation when the sum of $a$ and $b$ can be stored (= calculated without overflow) in the original type. For primitive integers, the computation roughly amounts to loading the values into registers, performing a single add followed by a right shift by 1. The exact assembly can be found on [godbo.lt](https://godbolt.org/z/7Mzjvoe9P), where one can also run [llvm-mca](https://www.youtube.com/watch?v=Ku2D8bjEGXk) on the assembly for the purpose of static performance analysis.

However, $a+b$ cannot be guaranteed to be computed without overflow.

# Saved work

[CAD97] gave the following opinion:

> Personally, I think only the first or the fifth options are really in the running to be picked. The first isn't commutative, but it has predictable and consistent behavior for ++, +-, -+, and --: bias towards the first argument. The fifth leans on the fact that integer division truncates toward zero for familiarity, and gains commutativity, but loses the consistency that midpoint(x, y) == -midpoint(-x, -y).
>
> It's a trade-off. If always rounding toward zero can be done branchless and rounding toward a can't, that'd make me more likely to support rounding toward zero, but I still think rounding toward a is more useful. (Plus, if it inlines, you can just sort a and b on input to get the the rounding you want at no cost.)

[^1]: https://internals.rust-lang.org/t/average-function-for-primitives/14040

[CAD97]: https://internals.rust-lang.org/u/CAD97
[user16251]: https://internals.rust-lang.org/u/user16251