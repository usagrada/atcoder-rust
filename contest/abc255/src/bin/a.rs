#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; 2]; 2],
    }

    println!("{}", a[r - 1][c - 1]);
}
