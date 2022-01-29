pub trait MidpointViaBitwiseOpsExt {
    #[must_use]
    fn midpoint_via_bitwise_ops(&self, rhs: &Self) -> Self;
}

macro_rules! impl_midpoint_fn_for_t {
    () => {
        fn midpoint_via_bitwise_ops(&self, rhs_ref: &Self) -> Self {
            let (lhs, rhs) = (*self, *rhs_ref);
            let (half_lhs, half_rhs) = (lhs / 2, rhs / 2);
            let lsb_masked_bitwise_or = lhs & rhs & 0x1;
            sum_without_overflow!(half_lhs, half_rhs, lsb_masked_bitwise_or)
        }
    };
}

impl_for_all_prim_ints!(
    trait = MidpointViaBitwiseOpsExt,
    fn macro = impl_midpoint_fn_for_t
);
