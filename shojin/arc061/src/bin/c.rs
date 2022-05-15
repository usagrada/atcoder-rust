use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        s: String
    }
    let mut ans = 0u64;
    let n = s.len();
    for bit in 0..(1 << n) {
        if bit & 1 == 0 {
            continue;
        }
        let mut start = 0;
        let mut sum = 0usize;
        for i in 1..n {
            if (bit & (1 << i)) != 0 {
                let num = &s[start..i];
                // println!("num: {}", num);
                sum += num.parse::<usize>().unwrap();
                start = i;
            }
        }
        let num = &s[start..n];
        // println!("num: {}", num);
        sum += num.parse::<usize>().unwrap();
        ans += sum as u64;
        // println!("{}", sum);
    }
    println!("{}", ans);
}
