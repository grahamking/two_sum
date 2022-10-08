#![feature(portable_simd)]

use std::collections::HashMap;
use std::simd::{i32x16, Simd};

mod gen;
pub use gen::{gen, Policy};

// basic linear
pub fn two_sum_linear(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().enumerate() {
        for (j, right) in arr.iter().skip(i).enumerate() {
            if left + right == target {
                return (i, i + j);
            }
        }
    }
    (0, 0)
}

// linear using AVX-512 SIMD
pub fn two_sum_linear_simd(target: i32, arr: &[i32]) -> (usize, usize) {
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

// map
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

// basic linear but pad data so that each value is in it's own cache line
// Needs gen by 16 too
pub fn _two_sum_linear_pad(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().step_by(16).enumerate() {
        for (j, right) in arr.iter().step_by(16).skip(i).enumerate() {
            if left + right == target {
                return (i * 16, i * 16 + j * 16);
            }
        }
    }
    (0, 0)
}
