#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        s: String
    }
    let ans = s
        .chars()
        .fold(0, |acc, c| if c == '1' { acc + 1 } else { acc });
    println!("{}", ans);
}
