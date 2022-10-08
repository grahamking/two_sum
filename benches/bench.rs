use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use two_sum::{gen, two_sum_linear, two_sum_map};

pub fn two_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum");
    for i in (250..=3_000).step_by(250) {
        let t = gen(i);
        group.bench_function(BenchmarkId::new("linear", i), |b| {
            b.iter(|| black_box(two_sum_linear(t.target, &t.v)))
        });
        group.bench_function(BenchmarkId::new("map", i), |b| {
            b.iter(|| black_box(two_sum_map(t.target, &t.v)))
        });
    }
    group.finish();
}

criterion_group!(benches, two_sum);
criterion_main!(benches);
