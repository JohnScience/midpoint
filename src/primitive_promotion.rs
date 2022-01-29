use primitive_promotion::PrimitivePromotionExt as PP;

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

pub trait MidpointViaPrimitivePromotionExt: PP {
    #[must_use]
    fn midpoint_via_primitive_promotion(&self, rhs: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        fn midpoint_via_primitive_promotion(&self, rhs_ref: &Self) -> Self {
            // At the time of writing, explicit dereferencing is necessary because
            // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
            // and requires `#![feature(const_ops)]`
            //
            // Rust unstable book entry:
            // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
            let (lhs, rhs) = (
                *self as <Self as PP>::PrimitivePromotion,
                *rhs_ref as <Self as PP>::PrimitivePromotion,
            );
            ((lhs + rhs) / 2) as Self
        }
    };
}

// u128 and i128 don't have a primitive promotion
impl_for_prim_ints_with_prim_promotion!(
    trait = MidpointViaPrimitivePromotionExt,
    fn macro = impl_midpoint_fn_for_t
);

#[cfg(test)]
mod tests {
    use crate::MidpointViaPrimitivePromotionExt;

    #[test]
    fn midpoint_via_primitive_promotion_rounds_towards_zero_including_when_args_are_positive() {
        let result: i32 = 2.midpoint_via_primitive_promotion(&3);
        assert_eq!(result, 2);
    }

    #[test]
    fn midpoint_via_primitive_promotion_rounds_towards_zero_including_when_args_are_negative() {
        let result: i32 = (-3).midpoint_via_primitive_promotion(&-2);
        assert_eq!(result, -2);
    }
}
