use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n+1],
        mut c: [isize; n + m + 1]
    }
    // solve b
    // cx = ax * bx
    let mut b = vec![0isize; m + 1];

    for j in (0..=m).rev() {
        b[j] = c[n + j] / a[n];
        for i in 0..n {
            c[i + j] -= a[i] * b[j];
        }
    }

    println!("{}", b.iter().join(" "));
}
