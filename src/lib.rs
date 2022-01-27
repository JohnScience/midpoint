#![cfg_attr(any(doc, test, doctest, feature = "const_trait_impl"), feature(const_trait_impl))]

mod naive;
mod primitive_promotion;

// crate:: disambiguates primitive_promotion as the module import source (as opposed to crate import source)
// Note: crate in this context is THIS crate (akin to self:: for this module and super:: for parent module)
pub use crate::naive::NaiveMidpointExt;
pub use crate::primitive_promotion::MidpointViaPrimitivePromotionExt;

pub fn midpoint<T: Midpoint>(lhs: &T, rhs: &T) -> T {
    T::midpoint(lhs, rhs)
}

pub unsafe fn midpoint_assuming_lhs_leq_rhs<T: MidpointAssumingLhsLeqRhs>(lhs: &T, rhs: &T) -> T {
    T::midpoint_assuming_lhs_leq_rhs(lhs, rhs)
}

pub trait Midpoint {
    // The references will usually be optimized out along with inlining the function
    // (c) Kevin Reid, aka kpreid (https://github.com/kpreid)
    fn midpoint(&self, rhs: &Self) -> Self;
}

pub trait MidpointAssumingLhsLeqRhs {
    unsafe fn midpoint_assuming_lhs_leq_rhs(&self /*lhs*/, rhs: &Self) -> Self;
}

impl MidpointAssumingLhsLeqRhs for u8 {
    unsafe fn midpoint_assuming_lhs_leq_rhs(&self /*lhs*/, rhs: &Self) -> Self {
        debug_assert!(self <= rhs);
        self + (rhs-self) / 2
    }
}

impl Midpoint for u8 {
    fn midpoint(&self, rhs: &Self) -> Self {
        // TODO: consider using a LeqGraphElem type
        let (min, max) = (
            std::cmp::min(self,rhs),
            std::cmp::max(self, rhs)
        );
        unsafe { midpoint_assuming_lhs_leq_rhs(min, max) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
