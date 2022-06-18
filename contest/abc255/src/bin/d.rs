#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;
use superslice::Ext;

fn main() {
    input! {
        n: u64,
        // n: usize,
        q: usize,
        a: [u64; n],
        x: [u64; q],
    }

    let mut a = a;
    let mut sum_vec = vec![0; n as usize + 1];
    a.sort();
    for i in 0..n as usize {
        sum_vec[i + 1] = sum_vec[i] + a[i];
    }
    for xi in x {
        // let lower = LowerBounded::lower_bound(&a, &xi);
        let lower = a.lower_bound(&xi);
        // println!("{}", lower);
        if lower == n as usize {
            println!("{}", xi * n - sum_vec[n as usize]);
        } else {
            let lower_sum = xi * lower as u64 - sum_vec[lower];
            let upper_sum = (sum_vec[n as usize] - sum_vec[lower]) - xi * (n as u64 - lower as u64);
            println!("{}", lower_sum + upper_sum);
        }
        // println!("{}", ((xi * n) as i64 - sum as i64).abs());
    }
}
