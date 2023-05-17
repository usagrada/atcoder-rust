#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    let mut new_a = a.clone();
    for _ in 0..4 {
        let mut ok_flag = true;
        for i in 0..n {
            for j in 0..n {
                new_a[i][j] = a[n - j - 1][i];
                if new_a[i][j] == 1 {
                    if b[i][j] == 0 {
                        ok_flag = false;
                    }
                }
            }
        }

        if ok_flag {
            println!("Yes");
            return;
        }

        a = new_a.clone();
    }

    println!("No");

}
