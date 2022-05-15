#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, collections::HashSet};

fn main() {
    input! {
        h: u64,
        w: u64,
        n: usize,
        pair: [(u64, u64); n]
    }
    // let mut pair = pair;
    // pair.sort();
    let mut set = HashSet::new();
    let mut ans_vec = vec![0; 10];
    let sum = (h - 2) * (w - 2);
    ans_vec[0] = sum;
    let dx = vec![-2, -1, 0];
    let dy = vec![-2, -1, 0];

    let dx3 = vec![0, 1, 2];
    let dy3 = vec![0, 1, 2];
    for (a, b) in pair {
        // println!("{} {}", a, b);
        let a = a - 1;
        let b = b - 1;
        // check 5x5
        for dx in dx.iter() {
            for dy in dy.iter() {
                let x = a as i64 + dx;
                let y = b as i64 + dy;
                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as u64;
                let y = y as u64;
                if x + 2 >= h || y + 2 >= w {
                    continue;
                }
                let mut count = 0;
                // check 3x3
                for dx3 in dx3.iter() {
                    for dy3 in dy3.iter() {
                        let x3 = x + dx3;
                        let y3 = y + dy3;
                        if set.contains(&(x3, y3)) {
                            count += 1;
                        }
                    }
                }
                // println!("count: {}", count);
                // println!("vec: {:?}", &ans_vec);
                ans_vec[count + 1] += 1;
                ans_vec[count] -= 1;
            }
        }
        set.insert((a, b));
    }

    for ans in ans_vec {
        println!("{}", ans);
    }
}
