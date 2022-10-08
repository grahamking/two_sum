use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use two_sum::{gen, two_sum_linear, two_sum_linear_simd, two_sum_map, Policy};

pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum");
    for i in (50..=500).step_by(50) {
        let t = gen(i, Policy::Mid);
        group.bench_function(BenchmarkId::new("compare_linear", i), |b| {
            b.iter(|| black_box(two_sum_linear(t.target, &t.v)))
        });
        group.bench_function(BenchmarkId::new("compare_map", i), |b| {
            b.iter(|| black_box(two_sum_map(t.target, &t.v)))
        });
    }
    group.finish();
}

const REPS: usize = 300;

pub fn single(c: &mut Criterion) {
    let t = gen(REPS, Policy::Mid);
    c.bench_function("single_linear", |b| {
        b.iter(|| black_box(two_sum_linear(t.target, &t.v)))
    });
    c.bench_function("single_map", |b| {
        b.iter(|| black_box(two_sum_map(t.target, &t.v)))
    });
    c.bench_function("single_simd", |b| {
        b.iter(|| black_box(two_sum_linear_simd(t.target, &t.v)))
    });
}

criterion_group!(benches, compare, single);
criterion_main!(benches);
