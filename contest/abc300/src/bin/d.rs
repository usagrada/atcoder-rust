#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Roots;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: u64,
    }

    // 1000以下の素数の列挙
    let mut prime_vec = vec![];
    for i in 0..n {
        if i < 2 || i * i > n {
            continue;
        }
        let mut is_prime = true;
        for j in 2..i {
            if j * j > i {
                break;
            }
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_vec.push(i);
        }
    }

    let l = prime_vec.len();
    let mut ans = 0;
    for i in 0..l {
        for j in i + 1..l {
            let a = prime_vec[i];
            let b = prime_vec[j];

            let c2_max = n / (a * a * b);
            // let c_max = c2_max.sqrt();
            for ci in j + 1..l {
                let c = prime_vec[ci];
                if c * c > c2_max {
                    break;
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
