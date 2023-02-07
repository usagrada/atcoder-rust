#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }
    let mut note = vec![false; x + 1];
    note[0] = true;
    for (a, b) in ab {
        for i in (0..x).rev() {
            if !note[i] {
                continue;
            }
            for num in 1..=b {
                let index = i + a * num;
                if index > x {
                    break;
                }
                note[index] = true;
            }
        }
    }
    if note[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
