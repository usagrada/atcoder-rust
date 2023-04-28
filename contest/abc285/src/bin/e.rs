#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut cum_a = vec![0; n + 1];
    for i in 0..n {
        cum_a[i + 1] = cum_a[i] + a[i];
    }

    let mut ans = 0;
    for i in 1..n {
        // pattern1
        let unit_days = i * 2 + 1;
        if unit_days > n {
            break;
        }
        // i日間の平日 + 休日のセット
        let unit_sum = cum_a[i] * 2;
        let unit = (n / unit_days) as u64;
        let rest = n % unit_days;
        // 残り期間も休みを挟む
        let sum1 = {
            let mut sum = unit_sum * unit;
            if rest == 0 {
                sum
            } else {
                sum += cum_a[i + rest / 2] - cum_a[i];
                if rest % 2 == 1 {
                    sum += a[i + (rest + 1) / 2]
                }
                sum
            }
        };
        // 残り期間は休みを挟まない
        let sum2 = {
            let mut sum = unit_sum * unit;
            sum -= cum_a[i / 2];
            sum += cum_a[i] - cum_a[i / 2 + 1];
            let start = n - rest;
            for j in 0..rest {
                let index = start + j;
                let xi = min(i + j, n - index);
                sum += a[xi];
            }
            sum
        };

        // println!("{} {} {}", i, sum1, sum2);
        let mut sum = max(sum1, sum2);
        if unit <= 1 {
            sum = sum2;
        }
        ans = max(ans, sum);

        // pattern2
        let unit_days = i * 2 + 2;
        if unit_days > n {
            break;
        }
        // i日間の平日 + 休日のセット
        let unit_sum = cum_a[i] * 2 + a[i + 1];
        let unit = (n / unit_days) as u64;
        let rest = n % unit_days;
        // 残り期間も休みを挟む
        let sum1 = {
            let mut sum = unit_sum * unit;
            if rest == 0 {
                sum
            } else {
                sum += cum_a[i + rest / 2] - cum_a[i];
                if rest % 2 == 1 {
                    sum += a[i + (rest + 1) / 2]
                }
                sum
            }
        };
        // 残り期間は休みを挟まない
        let sum2 = {
            let mut sum = unit_sum * unit;
            sum -= cum_a[i / 2];
            sum += cum_a[i] - cum_a[i / 2 + 1];
            let start = n - rest;
            for j in 0..rest {
                let index = start + j;
                let xi = min(i + j, n - index);
                sum += a[xi];
            }
            sum
        };

        // println!("{} {} {}", i, sum1, sum2);
        let mut sum = max(sum1, sum2);
        if unit <= 1 {
            sum = sum2;
        }
        ans = max(ans, sum);

    }
    println!("{}", ans);
}
