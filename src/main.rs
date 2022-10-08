use two_sum::{gen, two_sum_linear, two_sum_map};

fn main() {
    let mut args = std::env::args().skip(1);
    let t = gen(args.next().unwrap().parse().unwrap());

    let res = match args.next().unwrap().as_str() {
        "l" => two_sum_linear(t.target, &t.v),
        "m" => two_sum_map(t.target, &t.v),
        _ => panic!("two_sum n l|m"),
    };
    println!("{} {res:?}={}", t.target, t.v[res.0] + t.v[res.1],);
}
