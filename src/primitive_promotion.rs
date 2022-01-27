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
    fn midpoint_via_primitive_promotion(&self, rhs: &Self) -> Self;
}

macro_rules! impl_for_t {
    (!const, $t:ty) => {
        impl MidpointViaPrimitivePromotionExt for $t {
            fn midpoint_via_primitive_promotion(&self, rhs_ref: &Self) -> Self {
                let lhs = *self as <Self as PP>::PrimitivePromotion;
                let rhs = *rhs_ref as <Self as PP>::PrimitivePromotion;
                ((lhs + rhs) / 2) as Self
            }
        }
    };
    (const, $t:ty) => {
        impl const MidpointViaPrimitivePromotionExt for $t {
            fn midpoint_via_primitive_promotion(&self, rhs_ref: &Self) -> Self {
                let lhs = *self as <Self as PP>::PrimitivePromotion;
                let rhs = *rhs_ref as <Self as PP>::PrimitivePromotion;
                ((lhs + rhs) / 2) as Self
            }
        }        
    };
}

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_prim_ints_with_prim_promotion {
    () => {
        impl_for_t!(const, u8);
        impl_for_t!(const, u16);
        impl_for_t!(const, u32);
        impl_for_t!(const, u64);
        impl_for_t!(const, i8);
        impl_for_t!(const, i16);
        impl_for_t!(const, i32);
        impl_for_t!(const, i64);
    }
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_prim_ints_with_prim_promotion {
    () => {
        impl_for_t!(!const, u8);
        impl_for_t!(!const, u16);
        impl_for_t!(!const, u32);
        impl_for_t!(!const, u64);
        impl_for_t!(!const, i8);
        impl_for_t!(!const, i16);
        impl_for_t!(!const, i32);
        impl_for_t!(!const, i64);
    }
}

impl_for_prim_ints_with_prim_promotion!();