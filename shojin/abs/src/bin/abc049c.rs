#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        s: String,
    }
    let mut flag = true;
    let mut s = s;
    let dic = vec!["dream", "dreamer", "erase", "eraser"];
    loop {
        if s.len() == 0 {
            flag = true;
            break;
        }
        let mut loopflag = true;
        for &word in dic.iter() {
            if s.ends_with(word) {
                s = s.trim_end_matches(word).to_string();
                loopflag = false;
            }
        }
        if loopflag {
            flag = false;
            break;
        }
    }
    if flag {
        println!("YES");
    } else {
        println!("NO");
    }
}
