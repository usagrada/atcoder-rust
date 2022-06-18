#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, mem};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut p = 0;
    let mut v = vec![false; 4];
    for i in 0..n {
        v[0] = true;
        // let mut new_v = v.clone();
        for j in (0..4).rev() {
            if v[j] {
                if j + a[i] < 4 {
                    v[j] = false;
                    v[j + a[i]] = true;
                } else {
                    v[j] = false;
                    p += 1;
                }
            }
        }
        // println!("{:?}", &new_v);
        // v = new_v.clone();
    }
    println!("{}", p);
}
