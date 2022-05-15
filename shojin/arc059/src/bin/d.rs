use itertools::Itertools;
use proconio::input;
use std::{cmp::*, collections::HashMap};

fn main() {
    input! {
        s: String,
    }
    let n = s.len();
    let mut siter = s.chars().peekable();
    let mut siter2 = s.chars().peekable();
    siter2.next();
    for i in 0..n {
        let c = siter.next().unwrap();
        siter2.next();
        if i + 1 < n {
            let cnext = siter.peek().unwrap();
            if c == *cnext {
                println!("{} {}", i + 1, i + 2);
                return;
            }
        }
        if i + 2 < n {
            let c2next = siter2.peek().unwrap();
            if c == *c2next {
                println!("{} {}", i + 1, i + 3);
                return;
            }
        }
    }
    println!("{} {}", -1, -1);
}
