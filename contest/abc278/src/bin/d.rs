#![allow(unused_imports)]
use proconio::input;
use itertools::Itertools;
use std::cmp::*;

fn main() {
    input!{
        n: usize,
        mut a: [i64; n],
        q: usize,
        queries: [Line; q],
    }
    println!("{:?}", a);

    let mut standard = 0;
    for query in queries {
        println!("query: {}", query);
        let q = query.split(" ").collect::<Vec<&str>>();
        println!("{:?}", q);
        if q[0] == "1" {
            let x = q[1].parse::<i64>().unwrap();
            standard = x;
        } else if q[0] == "2" {
            let i = q[1].parse::<usize>().unwrap() - 1;
            let x = q[2].parse::<i64>().unwrap();
            a[i] = x - standard;
        } else if q[0] == "3" {
            let i = q[1].parse::<usize>().unwrap() - 1;
            let output = standard + a[i];
            println!("{}", output);
        }
    }
}
