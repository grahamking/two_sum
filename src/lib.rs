#![feature(portable_simd)]

use std::collections::HashMap;
use std::simd::{LaneCount, Simd, SupportedLaneCount};

mod gen;
pub use gen::{gen, Policy};

// basic linear
#[inline(never)]
pub fn two_sum_linear_index(target: i32, arr: &[i32]) -> (usize, usize) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] + arr[j] == target {
                return (i, j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_linear_iter1(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().enumerate() {
        for (j, right) in arr[i..].iter().enumerate() {
            if left + right == target {
                return (i, i + j);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_linear_iter2(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().enumerate() {
        let right = target - left;
        if let Some(j) = arr.iter().skip(i).position(|x| *x == right) {
            return (i, i + j);
        }
    }
    (0, 0)
}

// linear using AVX-512 SIMD
pub fn two_sum_simd_512(target: i32, arr: &[i32]) -> (usize, usize) {
    two_sum_simd::<16>(target, arr)
}

// linear using AVX-256 SIMD
pub fn two_sum_simd_256(target: i32, arr: &[i32]) -> (usize, usize) {
    two_sum_simd::<8>(target, arr)
}

fn two_sum_simd<const LANES: usize>(target: i32, arr: &[i32]) -> (usize, usize)
where
    LaneCount<LANES>: SupportedLaneCount,
{
    for (i, left) in arr.iter().enumerate() {
        let need = target - left;

        let (before, simd_main, after) = arr[i..].as_simd::<LANES>();
        for j in 0..before.len() {
            if before[j] == need {
                return (i, i + j);
            }
        }

        let simd_need: Simd<i32, LANES> = Simd::splat(need);
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

        for j in 0..after.len() {
            if after[j] == need {
                return (i, i + j + before.len() + simd_main.len() * LANES);
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
            return (i, *j);
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
