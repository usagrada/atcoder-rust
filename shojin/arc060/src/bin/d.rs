#![allow(unused)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: u64,
        s: u64,
    }
    if s == 1 {
        println!("{}", n);
        return;
    }
    for b in 2..=s {
        // if (b * b.to_string().len() as u64) < s {
        //     // break;
        //     continue;
        // }
        // println!("{} {}", b, n);
        let res = f(b, n);
        if res == s {
            println!("{}", b);
            return;
        }
    }

    println!("-1");
}

fn f(b: u64, n: u64) -> u64 {
    if n == 1 {
        return b;
    }
    if n < b {
        n
    } else {
        f(b, n / b) + n % b
    }
}
