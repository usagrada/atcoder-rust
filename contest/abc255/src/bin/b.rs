#![allow(unused_imports)]
use itertools::Itertools;
use libm::sqrt;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
        z: [(isize, isize); n]
    };
    // let mut sorty = z.clone();
    // sorty.sort_by_key(|&(x, y)| y);
    // let mut sortx = z.clone();
    // sortx.sort_by_key(|&(x, y)| x);
    let mut not_spot = vec![];
    for i in 0..n {
        let mut flag = true;
        for j in 0..k {
            if i + 1 == a[j] {
                flag = false;
                break;
            }
        }
        if flag {
            not_spot.push(z[i]);
        }
    }
    // println!("{:?}", not_spot);
    let mut ans = 0f64;
    for to_spot in not_spot {
        let mut distance = std::f64::MAX;
        for &ai in a.iter() {
            let (aix, aiy) = z[ai as usize - 1];
            let (tx, ty) = to_spot;
            let d = sqrt(((aix - tx) * (aix - tx) + (aiy - ty) * (aiy - ty)) as f64);
            if d < distance {
                distance = d;
            }
        }
        if distance > ans {
            ans = distance;
        }
    }
    println!("{}", ans);
}
