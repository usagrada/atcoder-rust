use proconio::input;

fn main() {
  input!{
    n: usize,
    mut a: [usize; n],
    mut b: [usize; n],
  }

  a.sort();
  b.sort();
  let mut ans = 0;
  for i in 0..n {
    let diff = (a[i] as isize - b[i] as isize).abs();
    ans += diff;
  }
  println!("{}", ans);
}
