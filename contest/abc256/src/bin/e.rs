#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
        c: [u64; n],
    }
    // let topo = to n, &x, &c);
    // let p = vec![0; n];
    let mut edges = vec![];
    let mut ans: u64 = 0;
    let mut queue = std::collections::VecDeque::new();
    for i in 0..n {
        edges.push((i, x[i]-1));
        queue.push_back((i, i));
    }

    while let Some((start, now)) = queue.pop_front() {
        for &(v, w) in &edges {
            if v == now {
                queue.push_back((start, w));
                // if(Z)
            }
        }
    }
    println!("{}", ans);
}
