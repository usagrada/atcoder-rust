#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        a: usize,
        b: usize
    }
    if a * 2 == b || a * 2 + 1 == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
