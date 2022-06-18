#![allow(unused_imports)]
use itertools::{sorted, Itertools};
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a = a;
    a.sort();
    let (alice_sum, bob_sum) =
        a.iter()
            .rev()
            .enumerate()
            .fold((0, 0), |(alice, bob), (index, &value)| {
                if index % 2 == 0 {
                    (alice + value, bob)
                } else {
                    (alice, bob + value)
                }
            });
    println!("{}", alice_sum - bob_sum);
}
