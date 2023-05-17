#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;
use std::collections::{BinaryHeap, HashSet};

fn main() {
    input! {
        n:usize,
        q: usize,
    }

    const M: usize = 200_001;

    let mut boxes = vec![BinaryHeap::new(); n+1];
    let mut num_box_manager = vec![HashSet::new(); M];
    for _ in 0..q {
        input! {
            id: usize,
        }
        if id == 1 {
            input! {
                i: usize,
                j: usize,
            }
            boxes[j - 1].push(i);
            num_box_manager[i].insert(j - 1);
        }
        if id == 2 {
            input! {
                i: usize,
            }
            for x in boxes[i - 1].iter().sorted() {
                print!("{} ", x);
            }
            print!("\n");
        }
        if id == 3 {
            input! {
                i: usize,
            }

            for j in num_box_manager[i].iter().sorted() {
                print!("{} ", j + 1);
            }
            print!("\n")
        }
    }
}
