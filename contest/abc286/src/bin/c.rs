#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: String,
    }
    let mut count_same = vec![0; n];

    let mut ans = std::u64::MAX;
    for start in 0..n {
        for i in 0..n {
            let lindex = (i + start) % n;
            let rindex = (2 * n - i - 1 + start) % n;
            let sl = &s[lindex..lindex + 1];
            let sr = &s[rindex..rindex + 1];
            // println!("sl: {}, sr: {}, {}", sl, sr, sl == sr);
            if sl.to_string() == sr.to_string() {
                count_same[start] += 1;
            }
        }
        let cost = a * start as u64 + b * (n - count_same[start]) as u64 / 2;
        // println!("{} {} {}", start, cost, count_same[start]);
        ans = min(ans, cost);
    }
    println!("{}", ans)
}
