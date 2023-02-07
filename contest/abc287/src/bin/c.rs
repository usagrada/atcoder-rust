#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut point_vec = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (u, v) in uv {
        point_vec[u - 1].push(v - 1);
        point_vec[v - 1].push(u - 1);
        uf.unite(u - 1, v - 1);
    }
    let one_edge_point = point_vec
        .iter()
        .fold(0, |cur, x| if x.len() == 1 { cur + 1 } else { cur });
    let two_edge_point = point_vec
        .iter()
        .fold(0, |cur, x| if x.len() == 2 { cur + 1 } else { cur });

    let flag = (0..n).fold(true, |cur, value| uf.same(0, value) && cur);
    if one_edge_point == 2 && two_edge_point == n - 2 && flag {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[derive(Debug)]
pub struct UnionFind {
    n: usize,
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            n: n,
            // parents: parents
            parents: (0..n).collect(),
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.root(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) -> &Self {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return self;
        }
        self.parents[ry] = rx;
        self
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        rx == ry
    }
}
