#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, vec};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut ans = 0;
    for i in 0..=a {
        for j in 0..=b{
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == x {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
