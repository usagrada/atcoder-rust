use proconio::input;
use std::collections::HashMap;
fn main() {
  input! {
    n: usize,
    a: [usize; n]
  }
  let mut map = HashMap::new();
  for i in 0..n {
    *map.entry(a[i]).or_insert(0) += 1;
  }
  // println!("map: {:?}", map);

  let mut ans = 0;
  for (_key, value) in map.iter() {
    let othernum = n - value;
    ans += value * othernum;
  }
  ans = ans / 2;

  println!("{}", ans);
}
