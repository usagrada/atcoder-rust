#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        h: usize,
        m: usize,
    }
    for ha in 0..24 {
        for ma in 0..60 {
            let nowh = (h + ha + if m + ma >= 60 { 1 } else { 0 }) % 24;
            let nowm = (m + ma) % 60;
            let a = nowh / 10;
            let b = nowh % 10;
            let c = nowm / 10;
            let d = nowm % 10;
            let rev_h = a * 10 + c;
            let rev_m = b * 10 + d;
            if rev_h < 24 && rev_m < 60 {
                println!("{} {}", nowh, nowm);
                return;
            }
        }
    }
}
