use rand::prelude::*;

pub struct TestCase {
    pub target: i32,
    pub v: Vec<i32>,
}

pub enum Policy {
    Mid,
    Last,
    Rand,
}

// make a test case
pub fn gen(n: usize, pos: Policy) -> TestCase {
    match pos {
        Policy::Mid => do_gen(n, gen_mid_target),
        Policy::Last => do_gen(n, gen_last_target),
        Policy::Rand => do_gen(n, gen_rand_target),
    }
}

fn do_gen(n: usize, gen_target: fn(&[i32]) -> i32) -> TestCase {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(n as usize);
    let max = (i32::MAX / 2 - 1) as i32;
    for _ in 0..n {
        v.push(rng.gen_range(0..max));
    }
    //let target = gen_rand_target(rng, &v);
    //let target = gen_mid_target(&v);
    let target = gen_target(&v);
    TestCase { target, v }
}

fn gen_rand_target(v: &[i32]) -> i32 {
    let mut rng = rand::thread_rng();
    let p1: usize = rng.gen_range(0..v.len());
    let p2: usize = rng.gen_range(0..v.len());
    v[p1] + v[p2]
}

fn gen_last_target(v: &[i32]) -> i32 {
    assert!(v.len() > 2);
    v[v.len() - 1] + v[v.len() - 2]
}

fn gen_mid_target(v: &[i32]) -> i32 {
    assert!(v.len() > 2);
    let mid = v.len() / 2;
    v[mid - 1] + v[mid + 1]
}
