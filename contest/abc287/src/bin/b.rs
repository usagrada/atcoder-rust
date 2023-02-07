#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }

    let mut ans = 0;
    for si in s {
        for ti in t.iter() {
            if si.ends_with(ti) {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
