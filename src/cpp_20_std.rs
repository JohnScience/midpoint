use epui::EquisizedPrimitiveUnsignedIntExt as EPUI;

pub trait MidpointViaCpp20StdImplementationExt {
    #[must_use]
    fn midpoint_via_cpp_20_std_implementation(&self, b_ref: &Self) -> Self;
}

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

macro_rules! impl_midpoint_fn_for_t {
    () => {
        fn midpoint_via_cpp_20_std_implementation(&self, b_ref: &Self) -> Self {
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
            let (u_a,u_b) = (
                a as <Self as EPUI>::EquisizedPrimitiveUnsignedInt,
                b as <Self as EPUI>::EquisizedPrimitiveUnsignedInt,
            );
            if a > b {
                a.wrapping_sub(((u_a-u_b)/2) as Self) 
            } else {
                a.wrapping_add(((u_b-u_a)/2) as Self)
            }
        }
    };
}

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_t {
    ($trait_name:ident, $t:ty) => {
        impl const $trait_name for $t {
            impl_midpoint_fn_for_t!();
        }
    };
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_t {
    ($trait_name:ident, $t:ty) => {
        impl $trait_name for $t {
            impl_midpoint_fn_for_t!();
        }
    };
}

macro_rules! impl_for_all_prim_ints {
    ($trait_name:ident) => {
        impl_for_t!($trait_name,u8);
        impl_for_t!($trait_name,u16);
        impl_for_t!($trait_name,u32);
        impl_for_t!($trait_name,u64);
        impl_for_t!($trait_name,u128);
        impl_for_t!($trait_name,usize);
        impl_for_t!($trait_name,i8);
        impl_for_t!($trait_name,i16);
        impl_for_t!($trait_name,i32);
        impl_for_t!($trait_name,i64);
        impl_for_t!($trait_name,i128);
        impl_for_t!($trait_name,isize);
    };
}

impl_for_all_prim_ints!(MidpointViaCpp20StdImplementationExt);
