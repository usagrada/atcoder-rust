#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
        s: String,
    }
    println!("{} {}", a+b+c, s)
}
