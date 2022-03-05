use proconio::input;

fn main() {
  input! {
    n: usize,
    mut w:usize,
    mut c: [(usize, usize); n]
  }
  c.sort();
  c.reverse();
  // println!("{:?}", c);
  let mut ans = 0;
  for i in 0..n {
    let (a, b) = c[i];
    let num = if b < w { b } else { w };
    w -= num;
    ans += a * num;
    if w == 0 {
      break;
    }
  }
  println!("{}", ans);
}
