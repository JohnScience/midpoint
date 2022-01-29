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

pub trait MidpointViaNaiveMidpointDiffExt {
    unsafe fn midpoint_via_naive_midpoint_diff(&self, rhs_ref: &Self) -> Self;
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
            let arg_diff = (rhs - lhs) as <Self as EPSI>::EquisizedPrimitiveSignedInt;
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
}
