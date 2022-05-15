#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
#[allow(unused_imports)]
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: [usize; n]
    }
    const X: usize = 50;
    let mut dp = vec![vec![vec![0_u64; n * X + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for j in 1..=n {
        for k in 0..=n {
            for s in 0..=n * X {
                if s < x[j-1] {
                    dp[j][k][s] = dp[j - 1][k][s];
                } else {
                    if k > 0 {
                        dp[j][k][s] = dp[j - 1][k][s] + dp[j - 1][k - 1][s - x[j-1]];
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans += dp[n][i][i * a];
    }
    println!("{}", ans);
}
