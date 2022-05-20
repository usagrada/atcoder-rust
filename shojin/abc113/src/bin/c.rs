#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m : usize,
        pair: [(usize, u64); m]
    }
    let mut pair = pair.iter().enumerate().collect::<Vec<_>>();

    pair.sort_by(|a, b| (a.1).1.cmp(&(b.1).1));
    let mut note = vec![0; n + 1];
    let mut ans_vec = vec![];
    for &(index, &(p, _)) in pair.iter() {
        // println!("{} {}", p, y);
        note[p] += 1;
        let s = format!("{:0>6}{:0>6}", p, note[p]);
        ans_vec.push((index, s))
    }
    ans_vec.sort();
    for ans in ans_vec {
        println!("{}", ans.1);
    }
}
