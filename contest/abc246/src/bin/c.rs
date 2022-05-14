use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        k: u64,
        x: u64,
        mut a: [u64; n]
    }
    // a 円の商品に対して k 枚のクーポンを使用すると、その商品を max{a−kX,0} 円
    // 何枚使えるか
    let usable: Vec<u64> = a.iter().map(|ai| ai / x).collect();
    let usum = usable.iter().sum::<u64>();
    let asum = a.iter().sum::<u64>();
    if usum >= k {
        // println!("asum :{}", asum);
        let ans: u64 = asum - k * x;
        println!("{}", ans);
    } else {
        let mut new_a: Vec<u64> = a.iter().enumerate().map(|(i, ai)| ai - usable[i] * x).collect();
        new_a.sort();
        // println!("{:?}", new_a);
        let mut ans = 0u64;
        let index = k - usum;
        if index > n as u64 {
            println!("{}", ans);
        } else {
            let index = index as usize;
            for i in 0..(n - index) {
                ans += new_a[i];
            }
            println!("{}", ans);
        }
    }
}
