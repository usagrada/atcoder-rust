#![allow(unused_imports)]
use itertools::Itertools;
use num_bigint::BigInt;
use proconio::input;
use std::{cmp::*, collections::VecDeque};

const MOD_NUM: u64 = 998244353;

trait ModPow {
    fn modpow(self, exp: usize, m: Self) -> Self;
}

impl ModPow for u64 {
    // mod m で x の exp 乗を計算する
    fn modpow(self, mut n: usize, m: Self) -> Self {
        let mut res: u64 = 1;
        let mut x = self % m;
        while n > 0 {
            if n & 1 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            n >>= 1;
        }
        res
    }
}

fn main() {
    input! {
        q: usize
    }

    let mut s = 1;
    let mut num_vec = VecDeque::new();
    num_vec.push_back(1);

    for _ in 0..q {
        input! {
            id: usize
        }

        if id == 1 {
            input! {
                x: u64
            }
            s = s * 10 + x;
            s %= MOD_NUM;
            num_vec.push_back(x);
        } else if id == 2 {
            let l = num_vec.len();
            s = (s + 10*MOD_NUM - num_vec[0] * 10u64.modpow(l - 1, MOD_NUM)) % MOD_NUM;
            num_vec.pop_front();
        } else if id == 3 {
            println!("{}", s);
        }
    }
}
