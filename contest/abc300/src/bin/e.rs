#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: u64,
    }

    // 1 から 6 で何回割り切れるか計算する
    let div_list = vec![2, 3, 5];
    let div_count: Vec<u64> = div_list.iter().map(|i| n / i).collect();
}
