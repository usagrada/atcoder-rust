#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        s: String,
    }

    let mut good_flag = false;
    let mut bad_flag = false;
    for c in s.chars() {
        if c == 'o' {
            good_flag = true;
        } else if c == 'x' {
            bad_flag = true;
        }
    }

    if !bad_flag & good_flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
