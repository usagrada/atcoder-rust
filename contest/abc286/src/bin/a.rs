#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        a: [usize; n],
    }

    let mut b = a.clone();
    for i in p..=q {
        b[i] = a[i - p + r];
    }
    for i in r..=s {
        b[i] = a[i - r + p];
    }
    println!("{}", b.iter().join(" "));
}
