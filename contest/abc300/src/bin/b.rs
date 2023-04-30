#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        h: usize,
        w:usize,
        a: [String; h],
        b: [String; h],
    }

    for s in 0..h {
        for t in 0..w {
            let mut ans_flag = true;
            'bloop: for i in 0..h {
                for j in 0..w {
                    let bi = (i + s) % h;
                    let bj = (j + t) % w;
                    if a[i][j..j + 1] != b[bi][bj..bj + 1] {
                        ans_flag = false;
                        break 'bloop;
                    }
                }
            }
            if ans_flag {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
