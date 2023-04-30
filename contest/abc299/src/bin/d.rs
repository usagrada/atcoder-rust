#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::cmp::*;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = LineSource::new(stdin.lock());

    input!{
        from &mut stdin,
        n: usize
    }


    // s: 0*****1
    // 2分探索して、0から1に変化するところを探す
    let mut left = 0;
    let mut right = n;
    let mut mid = (left + right) / 2;
    let mut count = 0;
    loop {
        count += 1;
        if count > 20 {
            return;
        }
        println!("? {}", mid);
        input! {
            from &mut stdin,
            c: String
        }
        if c == "1" {
            right = mid;
        } else {
            left = mid + 1;
        }
        if right - left < 1 {
            println!("! {}", left - 1);
            return;
        }
        mid = (left + right) / 2;
    }
}
