use rand::prelude::*;
use std::collections::HashMap;

pub struct TestCase {
    pub target: i32,
    pub v: Vec<i32>,
}

pub fn two_sum_linear(target: i32, arr: &[i32]) -> (usize, usize) {
    for (i, left) in arr.iter().step_by(16).enumerate() {
        for (j, right) in arr.iter().step_by(16).skip(i).enumerate() {
            if left + right == target {
                return (i * 16, i * 16 + j * 16);
            }
        }
    }
    (0, 0)
}

pub fn two_sum_map(target: i32, arr: &[i32]) -> (usize, usize) {
    let mut m = HashMap::new();
    for (i, val) in arr.iter().step_by(16).enumerate() {
        m.insert(val, i);
    }
    for (i, val) in arr.iter().step_by(16).enumerate() {
        if let Some(j) = m.get(&(target - val)) {
            return (i * 16, *j * 16);
        }
    }
    (0, 0)
}

// make a test case
pub fn gen(n: usize) -> TestCase {
    const GAP: [i32; 15] = [0i32; 15];
    let mut rng = rand::thread_rng();
    // standard: let mut v = Vec::with_capacity(n as usize);
    let mut v = Vec::with_capacity(n as usize * 16);
    let max = (i32::MAX / 2 - 1) as i32;
    for _ in 0..n {
        v.push(rng.gen_range(0..max));
        // standard: remove next line
        v.extend(&GAP);
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
    // standard: v[mid - 1] + v[mid + 1]
    v[mid - 16] + v[mid + 16]
}
