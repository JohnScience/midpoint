// At the time of writing, the code below is impossible because const trait bounds in
// constant functions are not a thing. #![feature(const_fn_trait_bound)] enables
// only (non-const) trait bounds even in conjunction with #![feature(const_trait_impl)].
//
// ```rust
// #[inline(always)]
// pub const unsafe fn naive_midpoint<T: const NaiveMidpointExt>(lhs: &T, rhs: &T) -> T {
//     lhs.naive_midpoint(rhs)
// }
//

pub trait NaiveMidpointExt {
    unsafe fn naive_midpoint(&self, rhs_ref: &Self) -> Self;
}

macro_rules! impl_for_t {
    (!const, $t:ty) => {
        impl NaiveMidpointExt for $t {
            unsafe fn naive_midpoint(&self, rhs: &Self) -> Self {
                (self + rhs) / 2
            }
        }        
    };
    (const, $t:ty) => {
        impl const NaiveMidpointExt for $t {
            unsafe fn naive_midpoint(&self, rhs_ref: &Self) -> Self {
                // At the time of writing, explicit dereferencing is necessary because
                // `<&u8 as Add<&u8>>::add` is not yet stable as a const fn
                // and requires `#![feature(const_ops)]`
                //
                // Rust unstable book entry:
                // https://doc.rust-lang.org/beta/unstable-book/library-features/const-ops.html
                let (lhs, rhs) = (*self, *rhs_ref);
                (lhs + rhs) / 2
            }
        }        
    };
}

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_all_prim_ints {
    () => {
        impl_for_t!(const, u8);
        impl_for_t!(const, u16);
        impl_for_t!(const, u32);
        impl_for_t!(const, u64);
        impl_for_t!(const, u128);
        impl_for_t!(const, usize);
        impl_for_t!(const, i8);
        impl_for_t!(const, i16);
        impl_for_t!(const, i32);
        impl_for_t!(const, i64);
        impl_for_t!(const, i128);
        impl_for_t!(const, isize);
    }
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_all_prim_ints {
    () => {
        impl_for_t!(!const, u8);
        impl_for_t!(!const, u16);
        impl_for_t!(!const, u32);
        impl_for_t!(!const, u64);
        impl_for_t!(!const, u128);
        impl_for_t!(!const, usize);
        impl_for_t!(!const, i8);
        impl_for_t!(!const, i16);
        impl_for_t!(!const, i32);
        impl_for_t!(!const, i64);
        impl_for_t!(!const, i128);
        impl_for_t!(!const, isize);
    }
}

impl_for_all_prim_ints!();