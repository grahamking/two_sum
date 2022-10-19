#![feature(portable_simd)]

use std::collections::HashMap;
use std::simd::{Simd, SimdPartialEq, ToBitMask};

mod gen;
pub use gen::{gen, Policy, TestCase};

// basic linear
#[inline(never)]
pub fn two_sum_linear_index(target: i32, arr: &[i32]) -> (usize, usize) {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_linear_iter1(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().enumerate() {
        for (j, right) in arr[(i + 1)..].iter().enumerate() {
            if left + right == target {
                return (i, i + 1 + j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_linear_iter2(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().enumerate() {
        let right = target - left;
        if let Some(j) = arr.iter().skip(i + 1).position(|x| *x == right) {
            return (i, i + 1 + j);
        }
    }
    (0, 0)
}

// linear using AVX-512 SIMD
pub fn two_sum_simd_512(target: i32, arr: &[i32]) -> (usize, usize) {
    two_sum_simd(target, arr)
}

// linear using AVX-256 SIMD
// commented out because the new ToBitMask and const generics don't work together any more
// maybe because of this: https://github.com/rust-lang/portable-simd/pull/239
//pub fn two_sum_simd_256(target: i32, arr: &[i32]) -> (usize, usize) {
//    two_sum_simd::<8>(target, arr)
//}

fn two_sum_simd(target: i32, arr: &[i32]) -> (usize, usize) {
    const LANES: usize = 16;
    for (i, left) in arr.iter().enumerate() {
        let need = target - left;

        let (before, simd_main, after) = arr[(i + 1)..].as_simd::<LANES>();
        for j in 0..before.len() {
            if before[j] == need {
                return (i, i + 1 + j);
            }
        }

        let simd_need: Simd<i32, LANES> = Simd::splat(need);
        for (chunk_num, chunk) in simd_main.iter().enumerate() {
            let mask = chunk.simd_eq(simd_need);
            if mask.any() {
                // found it
                let j = mask.to_bitmask().trailing_zeros() as usize;
                return (i, i + 1 + before.len() + chunk_num * LANES + j);
            }
        }

        for j in 0..after.len() {
            if after[j] == need {
                return (i, i + 1 + j + before.len() + simd_main.len() * LANES);
            }
        }
    }
    (0, 0)
}

// map
pub fn two_sum_map(target: i32, arr: &[i32]) -> (usize, usize) {
    let mut m = HashMap::with_capacity(arr.len());
    for i in 0..arr.len() {
        m.insert(arr[i], i);
    }
    for i in 0..arr.len() {
        if let Some(j) = m.get(&(target - arr[i])) {
            if i != *j {
                return (i, *j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_map_iter(target: i32, arr: &[i32]) -> (usize, usize) {
    let mut m = HashMap::with_capacity(arr.len());
    for (i, val) in arr.iter().enumerate() {
        m.insert(val, i);
    }
    for (i, val) in arr.iter().enumerate() {
        if let Some(j) = m.get(&(target - val)) {
            if i != *j {
                return (i, *j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_map_onepass(target: i32, arr: &[i32]) -> (usize, usize) {
    let mut m = HashMap::with_capacity(arr.len());
    for i in 0..arr.len() {
        if let Some(j) = m.get(&(target - arr[i])) {
            return (i, *j);
        }
        m.insert(arr[i], i);
    }
    (0, 0)
}

// basic linear but pad data so that each value is in it's own cache line
// Needs gen by 16 too
pub fn _two_sum_linear_pad(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().step_by(16).enumerate() {
        for (j, right) in arr.iter().step_by(16).skip(i + 1).enumerate() {
            if left + right == target {
                return (i * 16, i * 16 + j * 16);
            }
        }
    }
    (0, 0)
}
