use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize,
    }
    let mut x4 = x3;
    let mut y4 = y3;
    if x1 == x2 {
        x4 = x3;
    }
    if x2 == x3 {
        x4 = x1;
    }
    if x3 == x1 {
        x4 = x2;
    }

    if y1 == y2 {
        y4 = y3;
    }
    if y2 == y3 {
        y4 = y1;
    }
    if y3 == y1 {
        y4 = y2;
    }
    println!("{} {}", x4, y4);
}
