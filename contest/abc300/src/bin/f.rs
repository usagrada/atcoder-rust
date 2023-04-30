#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m: u64,
        k: u64,
        s: String,
    }

    let x_count = s.chars().filter(|&c| c == 'x').count();
    if x_count as u64 > k {
        // s に含まれる x の数が k より多い場合は、
        // 高々1つの文字列sの範囲内で置き換えた方が良い
        let mut ans = 0;
        for start in 0..n {
            if &s[start..start + 1] == "x" {
                continue;
            }
            let mut count = 0;
            let mut change_num = 0;
            for i in 0..n {
                if m == 1 {
                    if start + i > n {
                        break;
                    }
                }
                let si = (start + i) % n;
                if &s[si..si + 1] == "x" {
                    if change_num == k {
                        // 既に k 回置き換えているので、終了
                        break;
                    }
                    change_num += 1
                }
                count += 1;
            }
            ans = max(ans, count);
        }
        println!("{}", ans);
        return;
    }

    let kx = k / x_count as u64;
    let ans = if kx >= 2 { n as u64 * (kx - 2) } else { 0 };

    // 左からn個目のxの位置を求める
    let mut x_pos = vec![];
    for i in 0..n {
        if &s[i..i + 1] == "x" {
            x_pos.push(i);
        }
    }
    let kx_mod = k % (2 * x_count) as u64;
    // 左からkx_mod個数個のxを置き換える
    let mut ans_rem = 0;
    for right in 0..kx_mod {
        let left = kx_mod - right;
        let left_pos = x_pos[x_count - ((left as usize - 1) % x_count) - 1];
        let right_pos = x_pos[(right as usize + 1) % x_count];

        let mut ans_rem_tmp = (n - left_pos) + right_pos;
        if left >= x_count as u64 || right >= x_count as u64 {
            ans_rem_tmp += n;
        } else if left == 0 || left == x_count as u64 {
            // 右端の連続するoの分をカウント
            ans_rem_tmp += n - x_pos[0];
        } else if right == 0 || right == x_count as u64 {
            // 左端の連続するoの分をカウント
            ans_rem_tmp += x_pos[0];
        }

        ans_rem = max(ans_rem, ans_rem_tmp as u64);
        // eprintln!("{} {}: {}", left, right, ans_rem)
    }
    println!("{}", ans + ans_rem);
}
