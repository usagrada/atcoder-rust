#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        mut xyr: [(usize, usize, usize); n],
    }
    xyr.sort();

    let length = 10000;
    let position = vec![(0, 0, 0, 0); n];
    let area_info = vec![vec![0; length]; length];

    let mut dp = vec![0; n];
    for &(x, y, r) in xyr.iter() {
        for i in 0..length {

        }
    }
}

fn calc_point(xyr: Vec<(usize, usize, usize)>, pos_vec: Vec<(usize, usize, usize, usize)>) -> f64 {
    let n = pos_vec.len();
    let mut point = 0.0;
    for i in 0..n{
        let (x, y, r) = xyr[i];
        let (x1, y1, x2, y2) = pos_vec[i];
        let mut pi = 0.0;

        if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
            let s = (x2 - x1) * (y2 - y1);
            let min_rs = min(r, s) as f64;
            let max_rs = max(r, s) as f64;
            pi = 1.0 - (1.0 - min_rs / max_rs).powi(2);
        }
        point += pi;
    }
    point
}
