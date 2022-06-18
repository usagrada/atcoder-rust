#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [i64; n-1],
        x: [i64; m],
    }
    // a[n]
    let mut a = vec![0; n];
    // a_0 = 0 のときを計算する
    for i in 0..n - 1 {
        a[i + 1] = s[i] - a[i];
    }
    let mut a_odd: Vec<i64> = (0..n).filter(|&i| i % 2 == 1).map(|i| a[i]).collect(); // -a_0
    let mut a_even: Vec<i64> = (0..n).filter(|&i| i % 2 == 0).map(|i| a[i]).collect(); // +a_0
    a_odd.sort();
    a_even.sort();
    let mut ans = 0;
    for &xi in x.iter() {
        // a_odd.
        // a_even[0] = xi
        let a0 = xi - a_even[0];
        let mut cnt = 0;
        for (index, &ai) in a.iter().enumerate() {
            let tmp = if index % 2 == 0 { 1 } else { -1 };
            for &xj in x.iter() {
                if ai + a0 * tmp == xj {
                    cnt += 1;
                }
            }
        }
        ans = max(ans, cnt);

        let a0 = xi - a_odd[0];
        let mut cnt = 0;
        for (index, &ai) in a.iter().enumerate() {
            let tmp = if index % 2 == 0 { 1 } else { -1 };
            for &xj in x.iter() {
                if ai + a0 * tmp == xj {
                    cnt += 1;
                }
            }
        }
        ans = max(ans, cnt);
    }
    // let asave = a.clone();
    // for i in 0..n {
    //     for &xi in x.iter() {
    //         // a[i] = xi;
    //         // a_0 = xi - a[i];
    //         let a0 = xi - a[i];
    //         // a_0 = xi;
    //         let mut cnt = 0;
    //         for (index, &ai) in a.iter().enumerate() {
    //             let tmp = if index % 2 == 0 { 1 } else { -1 };
    //             for &xj in x.iter() {
    //                 if ai + a0 * tmp == xj {
    //                     cnt += 1;
    //                 }
    //             }
    //         }
    //         ans = max(ans, cnt);
    //     }
    // }
    println!("{}", ans);
}
