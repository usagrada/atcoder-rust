#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        plan: [(usize, isize, isize); n],
    }
    let (mut t, mut x, mut y) = (0, 0, 0);
    for i in 0..n {
        let (ti, xi, yi) = plan[i];
        let dt = ti - t;
        let dx = xi - x;
        let dy = yi - y;
        let pos_diff = (dx.abs() + dy.abs()) as usize;
        if pos_diff > dt {
            println!("No");
            return;
        }
        if pos_diff % 2 != dt % 2 {
            println!("No");
            return;
        }
        t = ti;
        x = xi;
        y = yi;
    }
    println!("Yes");
}
