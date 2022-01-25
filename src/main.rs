trait EquisizedPrimitiveUnsignedInt {
    type EquisizedPrimitiveUnsignedInt;
}

impl EquisizedPrimitiveUnsignedInt for u8 {
    type EquisizedPrimitiveUnsignedInt = u8;
}

impl EquisizedPrimitiveUnsignedInt for u16 {
    type EquisizedPrimitiveUnsignedInt = u16;
}

impl EquisizedPrimitiveUnsignedInt for u32 {
    type EquisizedPrimitiveUnsignedInt = u32;
}

impl EquisizedPrimitiveUnsignedInt for u64 {
    type EquisizedPrimitiveUnsignedInt = u64;
}

impl EquisizedPrimitiveUnsignedInt for u128 {
    type EquisizedPrimitiveUnsignedInt = u128;
}

impl EquisizedPrimitiveUnsignedInt for i8 {
    type EquisizedPrimitiveUnsignedInt = u8;
}

impl EquisizedPrimitiveUnsignedInt for i16 {
    type EquisizedPrimitiveUnsignedInt = u16;
}

impl EquisizedPrimitiveUnsignedInt for i32 {
    type EquisizedPrimitiveUnsignedInt = u32;
}

impl EquisizedPrimitiveUnsignedInt for i64 {
    type EquisizedPrimitiveUnsignedInt = u64;
}

impl EquisizedPrimitiveUnsignedInt for i128 {
    type EquisizedPrimitiveUnsignedInt = u128;
}

macro_rules! impl_for_t {
    ($fn_name:ident, $t:ty) => {
        pub const fn $fn_name(a_ref: &$t, b_ref: &$t) -> $t {
            use EquisizedPrimitiveUnsignedInt as EPUI;
            type U = <$t as EPUI>::EquisizedPrimitiveUnsignedInt;

            // Without explicit dereferencing, the function
            // wouldn't be const-qualified. Hungarian-like notation
            // is a necessary evil.
            let a = *a_ref;
            let b = *b_ref;
            // Similarly to C++, according to Rust's reference
            // (https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast),
            // the bit patterns (https://en.wikipedia.org/wiki/Type_conversion#:~:text=bit%20pattern)
            // of the arguments get safely reinterpreted
            // (https://en.wikipedia.org/wiki/Type_conversion#:~:text=interpretation%20of%20the%20bit%20pattern)
            // at compile time as the values of the equally-sized
            // unsigned  int via cast both in the narrow
            // (and the broader) sense
            // (https://en.wikipedia.org/wiki/Type_conversion#:~:text=The%20word%20cast)
            let u_a = a as U;
            let u_b = b as U;
            debug_assert!( u_a == unsafe { core::mem::transmute::<$t,U>(a) });
            debug_assert!( u_b == unsafe { core::mem::transmute::<$t,U>(b) });

            // Type limit comparisons are deemed useless
            // by the compiler but they demonstrate
            // the known invariants to the programmer
            #[allow(unused_comparisons)]
            if a > b {
                let u_a_sub_u_b = u_a-u_b;
                // Since a > b, a-b >= 1
                debug_assert!(u_a_sub_u_b >= 1);
                debug_assert!(u_a_sub_u_b <= U::MAX);
                // Ideally, there must be a constructive theorem for the lower bound
                //
                // In this case, the theorem that 1 is the result for
                // every pair (u_a,u_b) of the image of [0..U::MAX - 1] under
                // n ↦ (n+1,n)
                debug_assert!(U::MAX-0 == U::MAX);
                debug_assert!(
                    u_a_sub_u_b != U::MAX || u_a == U::MAX && u_b == 0
                );
                let u_midpoint_diff_down = u_a_sub_u_b/2;
                debug_assert!(u_midpoint_diff_down >= 0);
                debug_assert!(u_midpoint_diff_down <= U::MAX/2);
                // Ideally, there must be a constructive theorem for the lower bound
                debug_assert!(
                    u_midpoint_diff_down != U::MAX/2 || u_a == U::MAX && u_b == 0
                );
                let midpoint_diff_down = u_midpoint_diff_down as $t;
                // The assert below is impossible but const_format crate
                // allows to perform comparison of static core::str's
                //
                // debug_assert!(
                //     core::any::TypeId::of::<$t>() == core::any::TypeId::of::<U>() || {
                //         (U::MAX/2).to_string() == <$t>::MAX.to_string()
                //     }
                // );
                debug_assert!(
                    a-midpoint_diff_down == a-(((u_a - u_b)/2) as $t) 
                );
                a - midpoint_diff_down
            } else {
                let midpoint_diff_up = ((u_b-u_a)/2) as $t;
                a + midpoint_diff_up
            }
        }
    }
}

fn main() {
    println!("{}", midpoint_as_per_cpp_std_for_i8(&2,&8));
}

impl_for_t!(midpoint_as_per_cpp_std_for_u8, u8);
impl_for_t!(midpoint_as_per_cpp_std_for_u16, u16);
impl_for_t!(midpoint_as_per_cpp_std_for_u32, u32);
impl_for_t!(midpoint_as_per_cpp_std_for_u64, u64);
impl_for_t!(midpoint_as_per_cpp_std_for_u128, u128);
impl_for_t!(midpoint_as_per_cpp_std_for_i8, i8);
impl_for_t!(midpoint_as_per_cpp_std_for_i16, i16);
impl_for_t!(midpoint_as_per_cpp_std_for_i32, i32);
impl_for_t!(midpoint_as_per_cpp_std_for_i64, i64);
impl_for_t!(midpoint_as_per_cpp_std_for_i128, i128);