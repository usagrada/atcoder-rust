#![allow(unused_imports)]
use num::pow;
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        s: String,
    }
    let mut ans = 0;
    let len_s = s.len();
    for i in 1..len_s {
        ans += pow(26, i);
    }
    let mut index = 0;
    for c in s.chars() {
        index *= 26;
        index += c as usize - 'A' as usize;
    }
    ans += index + 1;
    println!("{}", ans);
}
