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
    ($( #[$const_qualifier:ident] )? $t:ty) => {
        impl $($const_qualifier)? MidpointViaPrimitivePromotionExt for $t {
            fn midpoint_via_primitive_promotion(&self, rhs_ref: &Self) -> Self {
                let lhs = *self as <Self as PP>::PrimitivePromotion;
                let rhs = *rhs_ref as <Self as PP>::PrimitivePromotion;
                ((lhs + rhs) / 2) as Self
            }
        }        
    };
}

macro_rules! impl_for_prim_ints_with_prim_promotion {
    ($( #[$const_qualifier:ident] )?) => {
        impl_for_t!($( #[$const_qualifier] )? u8);
        impl_for_t!($( #[$const_qualifier] )? u16);
        impl_for_t!($( #[$const_qualifier] )? u32);
        impl_for_t!($( #[$const_qualifier] )? u64);
        impl_for_t!($( #[$const_qualifier] )? i8);
        impl_for_t!($( #[$const_qualifier] )? i16);
        impl_for_t!($( #[$const_qualifier] )? i32);
        impl_for_t!($( #[$const_qualifier] )? i64);
    }
}

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
impl_for_prim_ints_with_prim_promotion!(#[const]);
#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
impl_for_prim_ints_with_prim_promotion!();