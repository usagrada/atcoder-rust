use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: usize,
    mut c: [(usize, usize); n],
  }  

  c.sort();
  // println!("{:?}", c);
  let min_a = c[0].0;
  let second_min_a = c[1].0;
  c.sort_by_key(|k| k.1);
  // println!("{:?}", c);
  let min_b = c[0].1;
  let second_min_b = c[1].1;

  if min_a == c[0].0 {
    // 同じ人の可能性がある場合
    let dif_a = second_min_a - min_a;
    let dif_b = second_min_b - min_b;

    if dif_a < dif_b {
      println!("{}", cmp::min(min_a+min_b, cmp::max(second_min_a, min_b)));
    }else {
      println!("{}", cmp::min(min_a+min_b ,cmp::max(min_a, second_min_b)));
    }
  }else {
    println!("{}", cmp::max(min_a, min_b));
  }
}
