#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    for num in 1..=n {
        let num_str = num.to_string();
        let keta_sum = num_str.chars().fold(0usize, |acc, c| {
            let c_num = c.to_digit(10).unwrap() as usize;
            acc + c_num
        });
        if a <= keta_sum && keta_sum <= b {
            ans += num;
        }
    }
    println!("{}", ans);
}
