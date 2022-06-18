#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut d = d;
    d.sort();
    let mut last_value = 0;
    let ans = d.iter().fold(0, |ans, &value| {
        if value > last_value {
            last_value = value;
            ans + 1
        } else {
            ans
        }
    });
    println!("{}", ans);
}
