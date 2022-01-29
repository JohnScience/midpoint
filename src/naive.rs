// At the time of writing, the code below is impossible because const trait bounds in
// constant functions are not a thing. #![feature(const_fn_trait_bound)] enables
// only (non-const) trait bounds even in conjunction with #![feature(const_trait_impl)].
//
// ```rust
// #[inline(always)]
// pub const unsafe fn naive_midpoint<T: const NaiveMidpointExt>(lhs: &T, rhs: &T) -> T {
//     lhs.naive_midpoint(rhs)
// }
// ```

pub trait NaiveMidpointExt {
    #[must_use]
    unsafe fn naive_midpoint(&self, rhs_ref: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
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
    };
}

impl_for_all_prim_ints!(
    trait = NaiveMidpointExt,
    fn macro = impl_midpoint_fn_for_t
);
