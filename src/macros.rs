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

macro_rules! impl_for_types {
    ($trait_name:ident, $fn_macro_name:ident, [$($t:ty),*]) => {
        $(
            impl_for_t!($trait_name, $fn_macro_name, $t);
        )*
    };
}

#[macro_export]
macro_rules! impl_for_all_prim_ints {
    (trait = $trait_name:ident, fn macro = $fn_macro_name:ident) => {
        impl_for_types!(
            $trait_name,
            $fn_macro_name,
            [
                u8, u16, u32, u64, u128, usize,
                i8, i16, i32, i64, i128, isize
            ]
        );
    };
}

#[macro_export]
macro_rules! impl_for_prim_ints_with_prim_promotion {
    (trait = $trait_name:ident, fn macro = $fn_macro_name:ident) => {
        impl_for_types!(
            $trait_name,
            $fn_macro_name,
            [
                u8, u16, u32, u64,
                i8, i16, i32, i64
            ]
        );
    };
}