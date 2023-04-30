#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        t: u64,
        c: [u64; n],
        r: [u64; n]
    }

    let is_contained_color_t = c.contains(&t);
    if is_contained_color_t {
        let mut ans = 0;
        let mut max_value = 0;
        for i in 0..n {
            if c[i] == t {
                if r[i] > max_value {
                    max_value = r[i];
                    ans = i + 1;
                }
            }
        }
        println!("{}", ans);
    } else {
        let mut ans = 0;
        let color = c[0];
        let mut max_value = 0;
        for i in 0..n {
            if c[i] == color {
                if r[i] > max_value {
                    max_value = r[i];
                    ans = i + 1;
                }
            }
        }
        println!("{}", ans);
    }
}
