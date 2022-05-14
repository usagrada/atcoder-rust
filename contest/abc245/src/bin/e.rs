use proconio::input;
use itertools::Itertools;
fn main() {
    input! {
      n: usize,
      m: usize,
      a: [usize; n],
      b: [usize; n],
      c: [usize; m],
      d: [usize; m]
    }
    let mut hako = vec![];
    for i in 0..n {
        hako.push((a[i], b[i]));
    }
    hako.sort();
    let mut choco = vec![];
    for i in 0..m {
        choco.push((c[i], d[i]));
    }
    choco.sort();
}
