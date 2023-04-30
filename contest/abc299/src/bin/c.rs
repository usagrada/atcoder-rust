#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        s: String
    }
    // レベルLのダンゴ列
    // L + 1 文字の文字列
    // o が L 個の左右一方に　- が 1 個
    // -ooo: L = 3
    // oo-: L = 2

    let contain_o = s.contains('o');
    // ハイフンを含む
    let contain_h = s.contains('-');
    if !contain_o || !contain_h {
        println!("{}", -1);
        return;
    }
    let mut ans = 1;
    let mut count = 0;

    for c in s.chars() {
        if c == 'o' {
            count += 1;
        } else {
            ans = max(ans, count);
            count = 0;
        }
    }
    ans = max(ans, count);

    println!("{}", ans)
}
