use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let mut min_cost = std::usize::MAX;
    for i in -101..101 {
        let mut cost = 0usize;
        for ai in a.iter() {
            cost += ((ai - i) * (ai - i)) as usize;
        }
        if cost < min_cost {
            min_cost = cost;
        }
    }
    println!("{}", min_cost);
}
