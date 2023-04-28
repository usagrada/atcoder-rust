#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
use itertools::Itertools;
use proconio::input;
use std::{cmp::*, collections::HashMap};

fn main() {
    input! {
        H: usize,
        W: usize,
        n: usize,
        h: usize,
        w: usize,
        a: [[usize; W]; H],
    }

    let mut m = HashMap::new();
    for i in 0..H {
        for j in 0..W {
            *m.entry(&a[i][j]).or_insert(0) += 1;
        }
    }
    let mut m2 = m.clone();
    for i in 0..h {
        for j in 0..w {
            *m2.get_mut(&a[i][j]).unwrap() -= 1;
        }
    }
    let mut ans_vec = vec![HashMap::new(); W - w + 1];
    ans_vec[0] = m2.clone();

    for j in 1..=W - w {
        ans_vec[j] = ans_vec[j - 1].clone();
        for k in 0..h {
            *ans_vec[j].get_mut(&a[k][j - 1]).unwrap() += 1;
            *ans_vec[j].get_mut(&a[k][j + w - 1]).unwrap() -= 1;
        }
    }
    println!(
        "{}",
        ans_vec
            .iter()
            .map(|x| x
                .iter()
                .filter(|(&k, &v)| v > 0)
                .collect::<Vec<_>>()
                .len()
                .to_string())
            .join(" ")
    );
    for i in 1..=H - h {
        for j in 0..=W - w {
            for k in 0..w {
                //
                *ans_vec[j].get_mut(&a[i - 1][j + k]).unwrap() += 1;
                *ans_vec[j].get_mut(&a[i + h - 1][j + k]).unwrap() -= 1;
            }
        }
        println!(
            "{}",
            ans_vec
                .iter()
                .map(|x| x
                    .iter()
                    .filter(|(&k, &v)| v > 0)
                    .collect::<Vec<_>>()
                    .len()
                    .to_string())
                .join(" ")
        );
    }
}
