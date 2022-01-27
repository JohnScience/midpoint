pub trait MidpointViaBitwiseOpsExt {
    fn midpoint_via_bitwise_ops(&self, rhs: &Self) -> Self;
}

// macro_rules! impl_for_t {
//     (!const, $t:ty) => {
//         impl MidpointViaBitwiseOpsExt for $t {
//             fn midpoint_via_bitwise_ops(&self, rhs: &Self) -> Self {
//                 // wrapping_add restricts the implementation,
//                 // unchecked_add is feature-gated and unsafe but
//                 // better reflects the known invariant (that the
//                 // addition won't overflow)
//                 unsafe{
//                     // >> compiler emits shr and sar for unsigned
//                     // and signed types, respectively
//                     (self >> 1)
//                         // The absolute value of n >> 1 is less than
//                         // or equal to the result of real division of n
//                         // by 2. Therefore, the absolute value of the
//                         // sum of near-halves cannot exceed neither
//                         // $t::MIN nor $t::MAX 
//                         .unchecked_add(rhs >> 1)
//                         // If the least significant bit of n s.t.
//                         // n=a=b is set to 0, "a & b & 0x1" is 0 and
//                         // the sum of halves cannot exceed n.
//                         // Otherwise, the corresponding right
//                         // shift discards the least significant digit
//                         // and then |2*(n >> 1)|=|n|-1.
//                         // Since "a & b & 0x1" is at most 1,
//                         // the expression can neither overflow nor
//                         // underflow.
//                         .unchecked_add(self & rhs & 0x1)
//                 }
//             }
//         }
//     };
//     (const, $t:ty) => {
//         impl const MidpointViaBitwiseOpsExt for $t {
//             fn midpoint_via_bitwise_ops(&self, rhs_ref: &Self) -> Self {
//                 let lhs = *self as <Self as PP>::PrimitivePromotion;
//                 let rhs = *rhs_ref as <Self as PP>::PrimitivePromotion;
//                 ((lhs + rhs) / 2) as Self
//             }
//         }        
//     };
// }

// macro_rules! impl_for_t {
//     ($const_qualifier:ident, $t:ty) => {
//         impl $const_qualifier MidpointViaBitwiseOpsExt for $t {
//             // ...
//         }
//     };
// }

// impl_for_t!(r# , u8);