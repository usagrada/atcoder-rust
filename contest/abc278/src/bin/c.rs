#![allow(unused_imports)]
use im_rc::hashmap;
use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::{
    cmp::*,
    collections::{HashMap, HashSet},
};

fn main() {
    input! {
        n: u64,
        q: usize,
        tab: [(usize, u64, u64); q],
    }

    let mut follow_check = HashMap::new();
    for i in 0..q {
        let (t, a, b) = tab[i];
        if t == 1 {
            follow_check.entry(a).or_insert(HashSet::new()).insert(b);
        } else if t == 2 {
            let value = follow_check.contains_key(&a);
            if value {
                follow_check.get_mut(&a).unwrap().remove(&b);
            }
        } else if t == 3 {
            let seta = follow_check.get(&a);
            let setb = follow_check.get(&b);
            if seta == None || setb == None {
                println!("No");
            } else {
                let seta = seta.unwrap();
                let setb = setb.unwrap();
                if seta.contains(&b) && setb.contains(&a) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
