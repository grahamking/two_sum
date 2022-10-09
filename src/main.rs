#![feature(bench_black_box)]

use core::hint::black_box;
use two_sum::{gen, two_sum_linear_index, two_sum_map, two_sum_simd_512, Policy};

const REPS: usize = 1_000_000;

fn main() {
    let mut args = std::env::args().skip(1);
    let t = gen(args.next().unwrap().parse().unwrap(), Policy::Mid);

    let func = match args.next().unwrap().as_str() {
        "l" => two_sum_linear_index,
        "s" => two_sum_simd_512,
        "m" => two_sum_map,
        _ => panic!("two_sum n l|s|m"),
    };
    let mut res = (0, 0);
    for _ in 0..REPS {
        res = func(t.target, &t.v);
        black_box(res);
    }

    println!("{} {res:?}={}", t.target, t.v[res.0] + t.v[res.1],);
}
