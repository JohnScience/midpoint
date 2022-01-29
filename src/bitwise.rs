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

/// Extension trait providing implementation of midpoint algorithm via
/// [bitwise operations](https://en.wikipedia.org/wiki/Bitwise_operation).
pub trait MidpointViaBitwiseOpsExt {
    /// Returns midpoint using algorithm using
    /// [bitwise operations](https://en.wikipedia.org/wiki/Bitwise_operation).
    /// For primitive integers, the result is rounded towards zero.
    /// 
    /// # Example
    /// 
    /// ```
    /// use midpoint::MidpointViaBitwiseOpsExt;
    /// 
    /// let result: i32 = (-7).midpoint_via_bitwise_ops(&-2);
    /// assert_eq!(result, -4);
    /// ```
    #[must_use]
    fn midpoint_via_bitwise_ops(&self /*lhs_ref*/, rhs_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        fn midpoint_via_bitwise_ops(&self /*lhs_ref*/, rhs_ref: &Self) -> Self {
            // At the time of writing, explicit dereferencing is necessary because
            // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
            // and requires `#![feature(const_ops)]`
            //
            // Rust unstable book entry:
            // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
            let (lhs, rhs) = (*self, *rhs_ref);
            // Equivalent to SAR or SHR depending on signedness
            let (half_lhs, half_rhs) = (lhs / 2, rhs / 2);
            let lsb_masked_bitwise_or = lhs & rhs & 0x1;
            sum_without_overflow!(half_lhs, half_rhs, lsb_masked_bitwise_or)
        }
    };
}

impl_for_all_prim_ints!(
    trait = MidpointViaBitwiseOpsExt,
    fn macro = impl_midpoint_fn_for_t
);

#[cfg(test)]
mod tests {
    use crate::MidpointViaBitwiseOpsExt;

    #[test]
    fn midpoint_via_bitwise_ops_rounds_towards_zero_including_when_args_are_positive() {
        let result: i32 = 2.midpoint_via_bitwise_ops(&3);
        assert_eq!(result, 2);
    }

    #[test]
    fn midpoint_via_bitwise_ops_rounds_towards_zero_including_when_args_are_negative() {
        let result: i32 = (-3).midpoint_via_bitwise_ops(&-2);
        assert_eq!(result, -2);
    }
}
