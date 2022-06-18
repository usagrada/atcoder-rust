#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        t: usize,
        case: [u64; t]
    }
    for n in case {
        let ns = n.to_string();
        let l = ns.len();
        let mut ans = "9".repeat(ns.len() - 1).parse::<u64>().unwrap();

        let mut repeat_vec = vec![];
        for i in 1..l {
            if l % i == 0 {
                repeat_vec.push(i);
            }
        }

        for i in repeat_vec {
            // i桁の繰り返し
            let val = ns[..i].parse::<u64>().unwrap();
            let val2 = val - 1;
            if val.to_string().repeat(l / i).parse::<u64>().unwrap() <= n {
                ans = max(val.to_string().repeat(l / i).parse::<u64>().unwrap(), ans);
            }
            if val2.to_string().repeat(l / i).parse::<u64>().unwrap() <= n {
                ans = max(val2.to_string().repeat(l / i).parse::<u64>().unwrap(), ans);
            }
        }

        println!("{}", ans);
    }
}
