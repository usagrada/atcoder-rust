#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        m: usize,
        road: [(usize, usize, usize); m]
    }
}

// s -> t への最短時間
// fn f(s,t,k)
