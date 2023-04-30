#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [String; h],
    }

    let n = min(h, w);
    for k in 0..n {
        let k = k + 1;

        let mut count = 0;
        for i in k..(h - k) {
            for j in k..(w - k) {
                if &c[i][j..j + 1] == "." {
                    continue;
                }
                // ばつ印を構成しているか確認
                let mut flag = true;
                for l in 1..=k {
                    if &c[i - l][j - l..j - l + 1] != "#"
                        || &c[i - l][j + l..j + l + 1] != "#"
                        || &c[i + l][j - l..j - l + 1] != "#"
                        || &c[i + l][j + l..j + l + 1] != "#"
                    {
                        flag = false;
                        break;
                    }
                }
                let is_inner_area = i >= k + 1 && i + k + 1 < h && j >= k + 1 && j + k + 1 < w;
                if is_inner_area
                    && &c[i - k - 1][j - k - 1..j - k] == "#"
                    && &c[i - k - 1][j + k + 1..j + k + 2] == "#"
                    && &c[i + k + 1][j - k - 1..j - k] == "#"
                    && &c[i + k + 1][j + k + 1..j + k + 2] == "#"
                {
                    flag = false;
                }
                if flag {
                    count += 1;
                }
            }
        }

        print!("{} ", count);
    }
    println!("");
}
