#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(
    any(doc, test, doctest, feature = "const_trait_impl"),
    feature(const_trait_impl)
)]
#![cfg_attr(
    any(doc, test, doctest, feature = "unchecked_math"),
    feature(unchecked_math)
)]
#![cfg_attr(
    any(doc, test, doctest, feature = "const_inherent_unchecked_arith"),
    feature(const_inherent_unchecked_arith)
)]

#[macro_use]
mod common_macros;

mod bitwise;
mod cpp_20_std;
mod naive;
mod naive_midpoint_diff;
mod primitive_promotion;

// crate:: disambiguates primitive_promotion as the module import source (as opposed to crate import source)
// Note: crate in this context is THIS crate (akin to self:: for this module and super:: for parent module)
pub use crate::bitwise::MidpointViaBitwiseOpsExt;
pub use crate::cpp_20_std::MidpointViaCpp20StdImplementationExt;
pub use crate::naive::NaiveMidpointExt;
pub use crate::naive_midpoint_diff::MidpointViaNaiveMidpointDiffExt;
pub use crate::primitive_promotion::MidpointViaPrimitivePromotionExt;
