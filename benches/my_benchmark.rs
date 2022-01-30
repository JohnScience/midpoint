use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midpoint::{
    MidpointViaBitwiseOpsExt, MidpointViaCpp20StdImplementationExt,
    MidpointViaNaiveMidpointDiffExt, MidpointViaPrimitivePromotionExt, NaiveMidpointExt,
};

macro_rules! bench_fn {
    (
        $group:ident,
        $t:ty,
        $fn_name:ident,
        $a:expr,
        $b:expr
    ) => {
        $group.bench_function(stringify!($t::$fn_name), |b| {
            b.iter(|| {
                let a: $t = black_box($a);
                let b: $t = black_box($b);
                a.$fn_name(&b)
            })
        });
    };
    (
        $group:ident,
        $t:ty,
        unsafe $fn_name:ident,
        $a:expr,
        $b:expr
    ) => {
        $group.bench_function(stringify!($t::$fn_name), |b| {
            b.iter(|| {
                let a: $t = black_box($a);
                let b: $t = black_box($b);
                unsafe { a.$fn_name(&b) }
            })
        });
    };
}

macro_rules! bench_all_fns {
    (
        $group:ident,
        u128,
        $a:expr,
        $b:expr
    ) => {
        bench_fn!($group, u128, midpoint_via_bitwise_ops, $a, $b);
        bench_fn!($group, u128, midpoint_via_cpp_20_std_implementation, $a, $b);
        bench_fn!($group, u128, unsafe midpoint_via_naive_midpoint_diff, $a, $b);
        bench_fn!($group, u128, unsafe naive_midpoint, $a, $b);
    };
    (
        $group:ident,
        i128,
        $a:expr,
        $b:expr
    ) => {
        bench_fn!($group, i128, midpoint_via_bitwise_ops, $a, $b);
        bench_fn!($group, i128, midpoint_via_cpp_20_std_implementation, $a, $b);
        bench_fn!($group, i128, unsafe midpoint_via_naive_midpoint_diff, $a, $b);
        bench_fn!($group, i128, unsafe naive_midpoint, $a, $b);
    };
    (
        $group:ident,
        $t:ty,
        $a:expr,
        $b:expr
    ) => {
        bench_fn!($group, $t, midpoint_via_bitwise_ops, $a, $b);
        bench_fn!($group, $t, midpoint_via_cpp_20_std_implementation, $a, $b);
        bench_fn!($group, $t, unsafe midpoint_via_naive_midpoint_diff, $a, $b);
        bench_fn!($group, $t, unsafe naive_midpoint, $a, $b);
        bench_fn!($group, $t, midpoint_via_primitive_promotion, $a, $b);
    };
}

macro_rules! benchmark_all_fns_for_t {
    ($benchmark_name:ident, $t:ident, $a:expr, $b:expr) => {
        fn $benchmark_name(c: &mut Criterion) {
            let mut group = c.benchmark_group(stringify!($t));
            bench_all_fns!(group, $t, 0, 20);
            group.finish();
        }
    };
}

benchmark_all_fns_for_t!(benchmark_for_u8, u8, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_u16, u16, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_u32, u32, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_u64, u64, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_u128, u128, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_i8, i8, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_i16, i16, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_i32, i32, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_i64, i64, 0, 20);
benchmark_all_fns_for_t!(benchmark_for_i128, i128, 0, 20);

criterion_group!(
    benches,
    benchmark_for_u8,
    benchmark_for_u16,
    benchmark_for_u32,
    benchmark_for_u64,
    benchmark_for_u128,
    benchmark_for_i8,
    benchmark_for_i16,
    benchmark_for_i32,
    benchmark_for_i64,
    benchmark_for_i128,
);
criterion_main!(benches);
