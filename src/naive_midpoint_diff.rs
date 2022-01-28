use epsi::EquisizedPrimitiveSignedIntExt as EPSI;

pub trait MidpointViaNaiveMidpointDiffExt {
    unsafe fn midpoint_via_naive_midpoint_diff_ext(&self, rhs_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        unsafe fn midpoint_via_naive_midpoint_diff_ext(&self, rhs_ref: &Self) -> Self {
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

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_t {
    ($t:ty) => {
        impl const MidpointViaNaiveMidpointDiffExt for $t {
            impl_midpoint_fn_for_t!();
        }
    };
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_t {
    ($t:ty) => {
        impl MidpointViaNaiveMidpointDiffExt for $t {
            impl_midpoint_fn_for_t!();
        }
    };
}

macro_rules! impl_for_all_prim_ints {
    () => {
        impl_for_t!(u8);
        impl_for_t!(u16);
        impl_for_t!(u32);
        impl_for_t!(u64);
        impl_for_t!(u128);
        impl_for_t!(usize);
        impl_for_t!(i8);
        impl_for_t!(i16);
        impl_for_t!(i32);
        impl_for_t!(i64);
        impl_for_t!(i128);
        impl_for_t!(isize);
    };
}

impl_for_all_prim_ints!();
