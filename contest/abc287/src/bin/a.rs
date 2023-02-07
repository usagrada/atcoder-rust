#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut count = 0;
    for s in ss {
        if s == "For" {
            count += 1;
        } else if s == "Against" {
            continue;
        }
    }
    if count * 2 > n {
        println!("Yes");
    } else {
        println!("No");
    }
}
