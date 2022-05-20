#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let length = n - (y - x);
    let max_len = min(length, max(n - y, x) + (y - x) / 2);

    let mut ans_vec = vec![0; n];
    for num in 1..n {
        ans_vec[num] = n - num;
    }
    // 距離が縮まった分
    for i in (max_len+1)..n {
        ans_vec[i+1-(y-x)] += ans_vec[i];
        ans_vec[i] = 0;
    }

    for ans in ans_vec.iter().skip(1) {
        println!("{}", ans);
    }
}
