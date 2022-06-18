#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        lr : [(usize, usize); n],
    }
    let mut lr = lr;
    lr.sort();
    let mut start_num = lr[0].0;
    let mut end_num = lr[0].1;
    let mut ans_vec = vec![];

    for i in 0..n {
        if lr[i].0 > end_num {
            ans_vec.push((start_num, end_num));
            start_num = lr[i].0;
            end_num = lr[i].1;
        } else {
            end_num = max(lr[i].1, end_num);
        }

        if i == n - 1 {
            ans_vec.push((start_num, end_num));
        }
    }
    for ans in ans_vec {
        println!("{} {}", ans.0, ans.1);
    }
}
