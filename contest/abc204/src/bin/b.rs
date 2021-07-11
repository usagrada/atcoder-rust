use proconio::input;

fn main() {
  input!{
    n: usize,
    a: [isize; n]
  }
  let mut ans = 0;
  for i in 0..n {
    ans += std::cmp::max(a[i] - 10, 0);
  }
  println!("{}", ans);
}
