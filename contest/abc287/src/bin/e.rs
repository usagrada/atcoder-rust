#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{
    cmp::*,
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut ans = vec![0; n];

    let group = (0..n).collect_vec();
    dfs(0, &group, &s, &mut ans);

    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn dfs(depth: usize, group: &Vec<usize>, s: &Vec<String>, ans: &mut Vec<usize>) {
    let mut next_group = vec![vec![]; 26];
    for &g in group.iter() {
        if s[g].len() == depth {
            ans[g] = depth;
            continue;
        }
        let c = &s[g][depth..depth + 1];
        let idx = (c.chars().next().unwrap() as u8 - 'a' as u8) as usize;
        next_group[idx].push(g);
    }
    for i in 0..26 {
        if next_group[i].len() > 1 {
            dfs(depth + 1, &next_group[i], s, ans);
        } else {
            for g in next_group[i].iter() {
                ans[*g] = depth;
            }
        }
    }
}
