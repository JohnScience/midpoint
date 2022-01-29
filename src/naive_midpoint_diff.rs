use epsi::EquisizedPrimitiveSignedIntExt as EPSI;

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

/// Extension trait providing implementation of midpoint algorithm via naive midpoint
/// difference
pub trait MidpointViaNaiveMidpointDiffExt {
    /// Returns midpoint using algorithm naively relying on the difference of arguments.
    /// For primitive integers, the result is rounded towards left argument.
    ///
    /// # Safety
    /// Difference of the second argument and the first argument must fit in
    /// <T as [EPSI]>::[EquisizedPrimitiveSignedInt](https://docs.rs/epsi/latest/epsi/trait.EquisizedPrimitiveSignedIntExt.html#associatedtype.EquisizedPrimitiveSignedInt)
    ///
    /// # Examples
    ///
    /// ## Correct usage:
    ///
    /// ```
    /// use midpoint::MidpointViaNaiveMidpointDiffExt;
    /// use epsi::EquisizedPrimitiveSignedIntExt as EPSI;
    /// 
    /// let rhs = <u32 as EPSI>::EquisizedPrimitiveSignedInt::MAX as u32;
    /// let lhs = 0u32;
    /// let result: u32 = unsafe { (lhs).midpoint_via_naive_midpoint_diff(&rhs) };
    /// assert_eq!(result, (i32::MAX/2) as u32);
    /// ```
    ///
    /// ## Incorrect usage:
    ///
    /// ```should_panic
    /// use midpoint::MidpointViaNaiveMidpointDiffExt;
    /// use epsi::EquisizedPrimitiveSignedIntExt as EPSI;
    /// 
    /// // Even after +1, the value fits in u32
    /// let rhs = <u32 as EPSI>::EquisizedPrimitiveSignedInt::MAX as u32 + 1;
    /// let lhs = 0u32;
    /// let result: u32 = unsafe { (lhs).midpoint_via_naive_midpoint_diff(&rhs) };
    /// // The assert below, however, is guaranteed to panic
    /// assert!(result == (i32::MAX/2) as u32 || result == 1 + (i32::MAX/2) as u32);
    /// ```
    unsafe fn midpoint_via_naive_midpoint_diff(&self /*lhs_ref*/, rhs_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        unsafe fn midpoint_via_naive_midpoint_diff(&self, rhs_ref: &Self) -> Self {
            // At the time of writing, explicit dereferencing is necessary because
            // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
            // and requires `#![feature(const_ops)]`
            //
            // Rust unstable book entry:
            // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
            let (lhs, rhs) = (*self, *rhs_ref);
            let arg_diff = rhs.wrapping_sub(lhs) as <Self as EPSI>::EquisizedPrimitiveSignedInt;
            let midpoint_diff = (arg_diff / 2) as Self;
            lhs + midpoint_diff
        }
    };
}

impl_for_all_prim_ints!(
    trait = MidpointViaNaiveMidpointDiffExt,
    fn macro = impl_midpoint_fn_for_t
);

#[cfg(test)]
mod tests {
    use crate::MidpointViaNaiveMidpointDiffExt;
    use epsi::EquisizedPrimitiveSignedIntExt as EPSI;

    #[test]
    fn midpoint_via_naive_midpoint_diff_rounds_towards_left_arg_including_when_args_are_positive() {
        let result: i32 = unsafe { 2.midpoint_via_naive_midpoint_diff(&3) };
        assert_eq!(result, 2);
    }

    #[test]
    fn midpoint_via_naive_midpoint_diff_rounds_towards_left_arg_including_when_args_are_negative() {
        let result: i32 = unsafe { (-3).midpoint_via_naive_midpoint_diff(&-2) };
        assert_eq!(result, -3);
    }

    #[test]
    fn midpoint_via_naive_midpoint_diff_may_return_incorrect_midpoint_for_args_with_diff_signs() {
        // i32::MAX - i32::MIN is intuitively > i32::MAX.
        let result = unsafe { (i32::MIN).midpoint_via_naive_midpoint_diff(&i32::MAX) };
        // Therefore, instead of the expected value (-1i32)
        assert!(result != -1);
        // the actual value is i32::MIN
        assert_eq!(result, i32::MIN);
    }

    #[test]
    fn midpoint_via_naive_midpoint_diff_work_when_rhs_sub_lhs_equals_max_of_equisized_int() {
        let rhs = <u32 as EPSI>::EquisizedPrimitiveSignedInt::MAX as u32;
        let lhs = 0u32;
        let result: u32 = unsafe { (lhs).midpoint_via_naive_midpoint_diff(&rhs) };
        assert_eq!(result, (i32::MAX/2) as u32);
    }

    #[test]
    fn midpoint_via_naive_midpoint_diff_work_when_rhs_sub_lhs_equals_min_of_equisized_int() {
        let rhs = i32::MIN;
        let lhs = 0i32;
        let result = unsafe { (lhs).midpoint_via_naive_midpoint_diff(&rhs) };
        assert_eq!(result, i32::MIN/2);
    }
}
