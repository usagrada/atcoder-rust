use itertools::Itertools;
use maplit::hashset;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: [usize; k],
    }

    for i in n..100000 {
        let s = i.to_string();
        let mut ok = true;
        for di in d.iter() {
            if s.contains(di.to_string().as_str()) {
                ok = false;
                break;
            }
        }
        if ok {
            println!("{}", s);
            return;
        }
    }
}
