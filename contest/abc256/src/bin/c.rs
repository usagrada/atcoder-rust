#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        h1: usize,
        h2: usize,
        h3: usize,
        w1: usize,
        w2: usize,
        w3: usize,
    }

    let mut ans = 0;
    for a11 in 1..31 {
        for a12 in 1..31 {
            for a21 in 1..31 {
                for a22 in 1..31 {
                    let a13 = a11 + a12 < h1;
                    let a23 = a21 + a22 < h2;
                    let a31 = a11 + a21 < w1;
                    let a32 = a12 + a22 < w2;
                    if a31 && a32 && a13 && a23 {
                        let num13 = h1 - a11 - a12;
                        let num23 = h2 - a21 - a22;
                        let num31 = w1 - a11 - a21;
                        let num32 = w2 - a12 - a22;
                        if num31 + num32 < h3 && num13 + num23 < w3 {
                            if h3 - num31 - num32 == w3 - num13 - num23 {
                                ans += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
