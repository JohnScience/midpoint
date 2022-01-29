#[cfg(any(doc, test, doctest, feature = "const_trait_impl"))]
macro_rules! impl_for_t {
    ($trait_name:ident, $fn_macro_name:ident, $t:ty) => {
        impl const $trait_name for $t {
            $fn_macro_name!();
        }
    };
}

#[cfg(not(any(doc, test, doctest, feature = "const_trait_impl")))]
macro_rules! impl_for_t {
    ($trait_name:ident, $fn_macro_name:ident, $t:ty) => {
        impl $trait_name for $t {
            $fn_macro_name!();
        }
    };
}

#[macro_export]
macro_rules! impl_for_all_prim_ints {
    (trait = $trait_name:ident, fn macro = $fn_macro_name:ident) => {
        impl_for_t!($trait_name, $fn_macro_name, u8);
        impl_for_t!($trait_name, $fn_macro_name, u16);
        impl_for_t!($trait_name, $fn_macro_name, u32);
        impl_for_t!($trait_name, $fn_macro_name, u64);
        impl_for_t!($trait_name, $fn_macro_name, u128);
        impl_for_t!($trait_name, $fn_macro_name, usize);
        impl_for_t!($trait_name, $fn_macro_name, i8);
        impl_for_t!($trait_name, $fn_macro_name, i16);
        impl_for_t!($trait_name, $fn_macro_name, i32);
        impl_for_t!($trait_name, $fn_macro_name, i64);
        impl_for_t!($trait_name, $fn_macro_name, i128);
        impl_for_t!($trait_name, $fn_macro_name, isize);
    };
}

#[macro_export]
macro_rules! impl_for_prim_ints_with_prim_promotion {
    (trait = $trait_name:ident, fn macro = $fn_macro_name:ident) => {
        impl_for_t!($trait_name, $fn_macro_name, u8);
        impl_for_t!($trait_name, $fn_macro_name, u16);
        impl_for_t!($trait_name, $fn_macro_name, u32);
        impl_for_t!($trait_name, $fn_macro_name, u64);
        impl_for_t!($trait_name, $fn_macro_name, i8);
        impl_for_t!($trait_name, $fn_macro_name, i16);
        impl_for_t!($trait_name, $fn_macro_name, i32);
        impl_for_t!($trait_name, $fn_macro_name, i64);
    };
}