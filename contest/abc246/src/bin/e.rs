// use itertools::Itertools;
use proconio::input;
// use std::cmp::*;

fn main() {
    input! {
        n: usize,
        ax: usize,
        ay: usize,
        bx: usize,
        by: usize,
        s: [String; n]
    }
    let ax = ax - 1;
    let ay = ay - 1;
    let bx = bx - 1;
    let by = by - 1;
    // println!("{:?}", s);
    if (ax + ay + bx + by) % 2 == 1 {
        println!("-1");
        return;
    }
    let mut dp = vec![vec![-1isize; n]; n];
    // let mut old_dp = dp.clone();
    dp[ax][ay] = 0;
    for i in 0..n {
        for (j, c) in s[i].chars().enumerate() {
            if c == '#' {
                dp[i][j] = -2;
            }
        }
    }

    let mut cnt = 0;
    let mut queue = vec![(ax, ay)];
    while queue.len() > 0 {
        cnt += 1;
        if dp[bx][by] != -1 {
            break;
        }
        let mut next_queue = vec![];
        for _ in 0..queue.len() {
            let z = queue.pop().unwrap();
            let (i, j) = z;

            // 左上
            let mut index = 1;
            while i >= index && j >= index {
                if dp[i - index][j - index] != -2 {
                    if dp[i - index][j - index] == -1 {
                        // 既に辿り着ける場所
                        dp[i - index][j - index] = cnt;
                        next_queue.push((i - index, j - index));
                    } else if dp[i - index][j - index] == cnt {
                        // 探索済み
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                index += 1;
            }

            // 右上
            let mut index = 1;
            while i + index < n && j >= index {
                if dp[i + index][j - index] != -2 {
                    if dp[i + index][j - index] == -1 {
                        // 既に辿り着ける場所
                        dp[i + index][j - index] = cnt;
                        next_queue.push((i + index, j - index));
                    } else if dp[i + index][j - index] == cnt {
                        // 探索済み
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                index += 1;
            }

            // 左下
            let mut index = 1;
            while i >= index && j + index < n {
                if dp[i - index][j + index] != -2 {
                    if dp[i - index][j + index] == -1 {
                        // 既に辿り着ける場所
                        dp[i - index][j + index] = cnt;
                        next_queue.push((i - index, j + index));
                    } else if dp[i - index][j + index] == cnt {
                        // 探索済み
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                index += 1;
            }

            // 右下
            let mut index = 1;
            while i + index < n && j + index < n {
                if dp[i + index][j + index] != -2 {
                    if dp[i + index][j + index] == -1 {
                        // 既に辿り着ける場所
                        dp[i + index][j + index] = cnt;
                        next_queue.push((i + index, j + index));
                    } else if dp[i + index][j + index] == cnt {
                        // 探索済み
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                index += 1;
            }
        }
        queue = next_queue;
    }
    println!("{}", dp[bx][by]);
}
