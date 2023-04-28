#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::{cmp::*, collections::VecDeque};

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = VecDeque::from(a);
    for _ in 0..k {
        ans.pop_front();
        ans.push_back(0);
    }
    let ans = ans.iter().join(" ");
    println!("{}", ans);
}
