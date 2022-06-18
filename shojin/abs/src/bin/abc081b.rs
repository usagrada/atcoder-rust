#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a = a;
    let mut ans = 0;
    'outer: loop {
        for i in 0..n {
            if a[i] % 2 == 0 {
                a[i] /= 2;
            } else {
                break 'outer;
            }
        }
        ans += 1;
    }
    println!("{}", ans);
}
