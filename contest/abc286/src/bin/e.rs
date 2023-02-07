#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::{cmp::*, collections::VecDeque};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        sn: [String; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let mut edges = vec![vec![]; n];
    for i in 0..n {
        let s = &sn[i];
        for (j, c) in s.chars().enumerate() {
            if c == 'Y' {
                edges[i].push(j);
            }
        }
    }
    // warshall_floyd
    let mut note = vec![vec![(0, 0); n]; n];
    for i in 0..n {
        for j in 0..n {
            if edges[i].contains(&j) {
                note[i][j] = (a[j], 1);
            } else {
                note[i][j] = (0, 1_000_000_000);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let (sum, cnt) = (note[i][k].0 + note[k][j].0, note[i][k].1 + note[k][j].1);
                if cnt < note[i][j].1 || (cnt == note[i][j].1 && note[i][j].0 < sum) {
                    note[i][j] = (sum, cnt);
                }
            }
        }
    }

    for i in 0..q {
        let (u, v) = uv[i];
        let flag = note[u][v].0 != 0;

        if flag {
            println!("{} {}", note[u][v].1, note[u][v].0 + a[u]);
        } else {
            println!("Impossible");
        }
    }
}
