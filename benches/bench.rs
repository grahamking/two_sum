use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use two_sum::{
    gen, two_sum_linear_index, two_sum_linear_iter1, two_sum_linear_iter2, two_sum_map,
    two_sum_map_iter, two_sum_map_onepass, two_sum_simd_512, Policy,
};

pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench");
    for i in (100..=1000).step_by(100) {
        let t = gen(i, Policy::Mid);
        /*
        group.bench_function(BenchmarkId::new("Linear", i), |b| {
            b.iter(|| black_box(two_sum_linear_index(t.target, &t.v)))
        });
        */
        group.bench_function(BenchmarkId::new("SIMD", i), |b| {
            b.iter(|| black_box(two_sum_simd_512(t.target, &t.v)))
        });
        group.bench_function(BenchmarkId::new("Map TwoPass", i), |b| {
            b.iter(|| black_box(two_sum_map(t.target, &t.v)))
        });
        group.bench_function(BenchmarkId::new("Map OnePass", i), |b| {
            b.iter(|| black_box(two_sum_map_onepass(t.target, &t.v)))
        });
    }
    group.finish();
}

const REPS: usize = 300;

pub fn single(c: &mut Criterion) {
    let t = gen(REPS, Policy::Mid);
    c.bench_function("single_linear", |b| {
        b.iter(|| black_box(two_sum_linear_index(t.target, &t.v)))
    });
    c.bench_function("single_map", |b| {
        b.iter(|| black_box(two_sum_map_iter(t.target, &t.v)))
    });
    c.bench_function("single_simd", |b| {
        b.iter(|| black_box(two_sum_simd_512(t.target, &t.v)))
    });
}

pub fn compare_linear(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_linear");
    let t = gen(REPS, Policy::Last);
    group.bench_function("linear_index", |b| {
        b.iter(|| black_box(two_sum_linear_index(t.target, &t.v)))
    });
    group.bench_function("linear_two_iter", |b| {
        b.iter(|| black_box(two_sum_linear_iter1(t.target, &t.v)))
    });
    group.bench_function("linear_iter_position", |b| {
        b.iter(|| black_box(two_sum_linear_iter2(t.target, &t.v)))
    });
    group.finish();
}

pub fn compare_map(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_map");
    let t = gen(REPS, Policy::Last);
    group.bench_function("map_index", |b| {
        b.iter(|| black_box(two_sum_map(t.target, &t.v)))
    });
    group.bench_function("map_iter", |b| {
        b.iter(|| black_box(two_sum_map_iter(t.target, &t.v)))
    });
    group.bench_function("map_onepass", |b| {
        b.iter(|| black_box(two_sum_map_onepass(t.target, &t.v)))
    });
    group.finish();
}

/*
pub fn compare_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_simd");
    let t = gen(REPS, Policy::Last);
    group.bench_function("simd_avx512", |b| {
        b.iter(|| black_box(two_sum_simd_512(t.target, &t.v)))
    });
    group.bench_function("simd_avx256", |b| {
        b.iter(|| black_box(two_sum_simd_256(t.target, &t.v)))
    });

    group.finish();
}
*/

criterion_group!(
    benches,
    compare,
    single,
    compare_linear,
    compare_map,
    //compare_simd
);
criterion_main!(benches);
