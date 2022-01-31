use epui::EquisizedPrimitiveUnsignedIntExt as EPUI;

// At the time of writing, the code below is impossible because const trait bounds in
// constant functions are not a thing. #![feature(const_fn_trait_bound)] enables
// only (non-const) trait bounds even in conjunction with #![feature(const_trait_impl)].
//
// ```rust
// #[inline(always)]
// pub const fn midpoint_via_primitive_promotion<T>(lhs: &T, rhs: &T) -> T
// where T: const MidpointViaPrimitivePromotionExt
// {
//     lhs.midpoint_via_primitive_promotion(rhs)
// }
// ```

/// Extension trait providing implementation of midpoint algorithm [as suggested for
/// C++20 standard library](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p0811r3.html).
/// For primitive integers, the result is rounded towards left argument.
pub trait MidpointViaCpp20StdImplementationExt {
    #[must_use]
    fn midpoint_via_cpp_20_std_implementation(&self /*a_ref*/, b_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        /// Returns midpoint using algorithm
        /// [as suggested for  C++20 standard library](http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p0811r3.html)
        /// . For primitive integers, the result is rounded towards left argument.
        ///
        /// # Examples
        ///
        /// ```
        /// use midpoint::MidpointViaCpp20StdImplementationExt;
        ///
        /// let result: i32 = (-3).midpoint_via_cpp_20_std_implementation(&-2);
        /// assert_eq!(result, -3);
        /// ```
        fn midpoint_via_cpp_20_std_implementation(&self /*a_ref*/, b_ref: &Self) -> Self {
            // The line below is impossible due to error[E0401]:
            //  "can't use generic parameters from outer function"
            //
            // type U = <Self as EPUI>::EquisizedPrimitiveUnsignedInt;

            // At the time of writing, explicit dereferencing is necessary because
            // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
            // and requires `#![feature(const_ops)]`
            //
            // Rust unstable book entry:
            // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
            let (a, b) = (*self, *b_ref);
            let (u_a, u_b) = (
                a as <Self as EPUI>::EquisizedPrimitiveUnsignedInt,
                b as <Self as EPUI>::EquisizedPrimitiveUnsignedInt,
            );
            if a > b {
                a.wrapping_sub(((u_a - u_b) / 2) as Self)
            } else {
                a.wrapping_add(((u_b - u_a) / 2) as Self)
            }
        }
    };
}

impl_for_all_prim_ints!(
    trait = MidpointViaCpp20StdImplementationExt,
    fn macro = impl_midpoint_fn_for_t
);

#[cfg(test)]
mod tests {
    use crate::MidpointViaCpp20StdImplementationExt;

    #[test]
    fn midpoint_via_cpp_20_std_implementation_rounds_towards_left_arg_including_when_args_are_positive(
    ) {
        let result: i32 = 2.midpoint_via_cpp_20_std_implementation(&3);
        assert_eq!(result, 2);
    }

    #[test]
    fn midpoint_via_cpp_20_std_implementation_rounds_towards_left_arg_including_when_args_are_negative(
    ) {
        let result: i32 = (-3).midpoint_via_cpp_20_std_implementation(&-2);
        assert_eq!(result, -3);
    }
}
