use itertools::Itertools;
use num_integer::{gcd, lcm};
use proconio::input;
use std::{cmp::*, u64::MAX};

fn main() {
    input! {
        t: usize,
        case: [(u64, u64, u64, u64, u64, u64); t]
    }

    for (n, a, b, x, y, z) in case {
        if a * x <= y && b * x <= z {
            println!("{}", n * x);
            continue;
        } else if a * x < y {
            let cnt_b = n / b;
            let ans = (n - cnt_b * b) * x + cnt_b * z;
            println!("{}", ans);
            continue;
        } else if b * x < z {
            let cnt_a = n / a;
            let ans = (n - cnt_a * a) * x + cnt_a * y;
            println!("{}", ans);
            continue;
        }
        // a, b で増やした方が効率が良い場合
        let lcm_ab = lcm(a, b);
        let cnt_ab = n / lcm_ab;
        let yz = if y * (lcm_ab / a) < z * (lcm_ab / b) {
            y
        } else {
            z
        };
        let ans = cnt_ab * yz;

        let rest_n = n - cnt_ab * lcm_ab;

        let rest_ans = min(
            (rest_n / a) * y + (rest_n % a) * x,
            (rest_n / b) * z + (rest_n % b) * x,
        );

        println!("{}", ans + rest_ans);
    }
}
