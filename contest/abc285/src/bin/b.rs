#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        s: String
    }
    for i in 1..n {
        let l = n - i;
        let mut ans = 0;
        for k in 0..l {
            if k + i >= n {
                ans = k + 1;
                break;
            }
            let sk = &s[k..k + 1];
            let ski = &s[k + i..k + i + 1];
            if sk.to_owned() == ski.to_owned() {
                break;
            }
            ans = k + 1;
        }
        println!("{}", ans);
    }
}
