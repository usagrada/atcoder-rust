#![allow(unused_imports)]
#![allow(unused_assignments)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        x: i64,
        a: i64,
        d: i64,
        n: i64,
    }
    let mut ans = 0;
    if x < a && d > 0 {
        ans = a - x;
    } else if x > a && d < 0 {
        ans = x - a;
    } else if d == 0 {
        ans = (x - a).abs();
    } else if d > 0 && x >= a + d * (n - 1) {
        ans = x - (a + d * (n - 1));
    } else if d < 0 && x <= a + d * (n - 1) {
        ans = a + d * (n - 1) - x;
    } else {
        let plainx = (x - a) / d;
        let dd = [-1, 0, 1];
        ans = min((x - a).abs(), (a + d * (n - 1) - x).abs());

        for &dx in dd.iter() {
            if plainx + dx >= 0 && plainx + dx < n {
                ans = min(ans, (x - a - d * (plainx + dx)).abs());
            }
        }
    }
    println!("{}", ans);
}
