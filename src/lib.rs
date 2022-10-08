#![feature(portable_simd)]

use rand::prelude::*;
use std::collections::HashMap;

use std::simd::{i32x16, Simd};

pub struct TestCase {
    pub target: i32,
    pub v: Vec<i32>,
}

pub fn two_sum_linear(target: i32, arr: &[i32]) -> (usize, usize) {
    const LANES: usize = 16;
    for (i, left) in arr.iter().enumerate() {
        let need = target - left;

        let (before, simd_main, after) = arr[i..].as_simd::<LANES>();
        if let Some(j) = before.iter().position(|&x| x == need) {
            return (i, i + j);
        }

        let simd_need: i32x16 = Simd::splat(need);
        for (chunk_num, chunk) in simd_main.iter().enumerate() {
            let mask = chunk.lanes_eq(simd_need);
            if mask.any() {
                // found it
                for j in 0..LANES {
                    if mask.test(j) {
                        return (i, i + before.len() + chunk_num * LANES + j);
                    }
                }
            }
        }

        if let Some(j) = after.iter().position(|&x| x == need) {
            return (i, i + j + before.len() + simd_main.len() * LANES);
        }
    }
    (0, 0)
}

pub fn two_sum_map(target: i32, arr: &[i32]) -> (usize, usize) {
    let mut m = HashMap::new();
    for (i, val) in arr.iter().enumerate() {
        m.insert(val, i);
    }
    for (i, val) in arr.iter().enumerate() {
        if let Some(j) = m.get(&(target - val)) {
            return (i, *j);
        }
    }
    (0, 0)
}

// make a test case
pub fn gen(n: usize) -> TestCase {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n as usize);
    let max = (i32::MAX / 2 - 1) as i32;
    for _ in 0..n {
        v.push(rng.gen_range(0..max));
    }
    //let target = gen_rand_target(rng, &v);
    let target = gen_mid_target(&v);
    TestCase { target, v }
}

fn _gen_rand_target<T: Rng>(mut rng: T, v: &[i32]) -> i32 {
    let p1: usize = rng.gen_range(0..v.len());
    let p2: usize = rng.gen_range(0..v.len());
    v[p1] + v[p2]
}

fn _gen_last_target(v: &[i32]) -> i32 {
    v[v.len() - 1] + v[v.len() - 2]
}

fn gen_mid_target(v: &[i32]) -> i32 {
    let mid = v.len() / 2;
    v[mid - 1] + v[mid + 1]
}
