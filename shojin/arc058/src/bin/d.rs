#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, collections::VecDeque};
const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize
    }
    let mut note = vec![vec![0u64; w]; h];
    let mut visited = vec![vec![false; w]; h];
    note[0][0] = 1;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    for i in h - a..h {
        for j in 0..b {
            visited[i][j] = true;
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if visited[x][y] {
            continue;
        }
        visited[x][y] = true;
        if x + 1 < h {
            note[x + 1][y] += note[x][y];
            note[x + 1][y] %= MOD;
            if !visited[x + 1][y] {
                queue.push_back((x + 1, y));
            }
        }
        if y + 1 < w {
            note[x][y + 1] += note[x][y];
            note[x][y + 1] %= MOD;
            if !visited[x][y + 1] {
                queue.push_back((x, y + 1));
            }
        }
    }
    // println!("{:?}", note);
    println!("{}", note[h - 1][w - 1] % MOD);
}
