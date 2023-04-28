#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, collections::HashMap};

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    }
    let mut hashmap = HashMap::new();
    // 文字列の種類数を表す
    let mut index: usize = 0;
    let mut edges = vec![];
    for (s, t) in st.iter() {
        if !hashmap.contains_key(&s) {
            hashmap.insert(s, index);
            index += 1;
        }
        if !hashmap.contains_key(&t) {
            hashmap.insert(t, index);
            index += 1;
        }
        let si = hashmap.get(&s).unwrap();
        let ti = hashmap.get(&t).unwrap();
        edges.push((*si, *ti));
    }

    let mut ans = true;
    let mut uf = UnionFind::new(index);
    for (s, t) in edges.iter() {
        if uf.same(*s, *t) {
            ans = false;
            break;
        }
        uf.unite(*s, *t);
    }
    
    if ans {
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
