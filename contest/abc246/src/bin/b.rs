use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        a: f64,
        b: f64
    }

    let r = (a * a + b * b).sqrt();
    // let r_ans = r + 1.0f64;
    println!("{} {}", a / r, b / r);
}
