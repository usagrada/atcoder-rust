#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
