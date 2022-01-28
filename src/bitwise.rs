pub trait MidpointViaBitwiseOpsExt {
    #[must_use]
    fn midpoint_via_bitwise_ops(&self, rhs: &Self) -> Self;
}

#[cfg(any(
    doc,
    test,
    doctest,
    all(feature = "unchecked_math", feature = "const_inherent_unchecked_arith")
))]
macro_rules! sum {
    (
        $half_lhs:ident,
        $half_rhs:ident,
        $lsb_masked_bitwise_or:ident
    ) => {
        unsafe {
            $half_lhs
                .unchecked_add($half_rhs)
                .unchecked_add($lsb_masked_bitwise_or)
        }
    };
}

#[cfg(not(any(
    doc,
    test,
    doctest,
    all(feature = "unchecked_math", feature = "const_inherent_unchecked_arith")
)))]
macro_rules! sum {
    (
        $half_lhs:ident,
        $half_rhs:ident,
        $lsb_masked_bitwise_or:ident
    ) => {
        $half_lhs
            .wrapping_add($half_rhs)
            .wrapping_add($lsb_masked_bitwise_or)
    };
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        fn midpoint_via_bitwise_ops(&self, rhs_ref: &Self) -> Self {
            let (lhs, rhs) = (*self, *rhs_ref);
            let (half_lhs, half_rhs) = (lhs / 2, rhs / 2);
            let lsb_masked_bitwise_or = lhs & rhs & 0x1;
            sum!(half_lhs, half_rhs, lsb_masked_bitwise_or)
        }
    };
}

#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_t {
    ($t:ty) => {
        impl const MidpointViaBitwiseOpsExt for $t {
            impl_midpoint_fn_for_t!();
        }
    };
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_t {
    ($t:ty) => {
        impl MidpointViaBitwiseOpsExt for $t {
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
