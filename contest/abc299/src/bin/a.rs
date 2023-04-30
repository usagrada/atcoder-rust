#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        s: String
    }

    let mut left = -1;
    let mut right = -1;
    let mut asterisk = -1;
    for (i, c) in s.chars().enumerate() {
        let index = i as isize;
        if c == '|' {
            if left == -1 {
                left = index;
            } else {
                right = index;
            }
        }
        if c == '*' {
            asterisk = index;
        }
    }
    if left < asterisk && asterisk < right {
        println!("in");
    } else {
        println!("out");
    }
}
