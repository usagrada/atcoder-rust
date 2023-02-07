#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut ans = String::new();
    let mut index = 0;
    loop {
        if index >= n {
            break;
        }
        if index == n - 1 {
            ans.push_str(&s[index..index+1]);
            break;
        }
        if &s[index..index+2] == "na" {
            ans.push_str("nya");
            index += 2;
        } else {
            ans.push_str(&s[index..index+1]);
            index += 1;
        }
    }
    println!("{}", ans);
}
