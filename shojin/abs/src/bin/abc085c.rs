#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        y: u64,
    }
    let y = (y / 1000u64) as usize;

    for i in 0..=n {
        for j in 0..=(n - i) {
            let hideyo = n - i - j;
            if 10 * i + 5 * j + hideyo == y {
                println!("{} {} {}", i, j, hideyo);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
