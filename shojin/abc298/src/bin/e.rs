#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        a: usize,
        b: usize
    }
    println!("{} {}", a, b);
    println!("hello");
    println!("{}", a - b);
    println!("world");
}
