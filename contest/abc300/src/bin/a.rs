#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    }

    for i in 0..n {
        if a + b == c[i] {
            println!("{}", i + 1);
            return;
        }
    }
}
