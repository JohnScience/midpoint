try_impl_unsafe_generic_const_fn_for_trait!(
    NaiveMidpointExt::naive_midpoint
);

/// Extension trait providing implementation of naive midpoint algorithm.
/// For primitive integers, the result is rounded towards zero.
pub trait NaiveMidpointExt {
    /// Returns midpoint using naive algorithm. For primitive integers, the result is
    /// rounded towards zero.
    ///
    /// # Safety
    /// The sum of arguments shoud fit into a variable of their type without overflow.
    ///
    /// # Examples
    ///
    /// ## Correct usage:
    ///
    /// ```
    /// use midpoint::NaiveMidpointExt;
    ///
    /// let result: i32 = unsafe { 2.naive_midpoint(&3) };
    /// assert_eq!(result, 2);
    /// ```
    ///
    /// ## Incorrect usage:
    ///
    /// ```no_run
    /// use midpoint::NaiveMidpointExt;
    ///
    /// let result = unsafe { u32::MAX.naive_midpoint(&u32::MAX) };
    /// // The assert below is not guaranteed to uphold
    /// assert_eq!(result, u32::MAX);
    /// ```
    #[must_use]
    unsafe fn naive_midpoint(&self /*lhs_ref*/, rhs_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        unsafe fn naive_midpoint(&self /*lhs_ref*/, rhs_ref: &Self) -> Self {
            // At the time of writing, explicit dereferencing is necessary because
            // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
            // and requires `#![feature(const_ops)]`
            //
            // Rust unstable book entry:
            // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
            let (lhs, rhs) = (*self, *rhs_ref);
            (lhs + rhs) / 2
        }
    };
}

impl_for_all_prim_ints!(
    trait = NaiveMidpointExt,
    fn macro = impl_midpoint_fn_for_t
);

#[cfg(test)]
mod tests {
    use crate::NaiveMidpointExt;

    #[test]
    fn naive_midpoint_rounds_towards_zero_including_when_args_are_positive() {
        let result: i32 = unsafe { 2.naive_midpoint(&3) };
        assert_eq!(result, 2);
    }

    #[test]
    fn naive_midpoint_rounds_towards_zero_including_when_args_are_negative() {
        let result: i32 = unsafe { (-3).naive_midpoint(&-2) };
        assert_eq!(result, -2);
    }
}
