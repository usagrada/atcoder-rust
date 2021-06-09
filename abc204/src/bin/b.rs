use proconio::input;

fn main() {
  input!{
    n: isize,
    a: [isize; n]
  }
  let mut ans = 0;
  for i in 0..n {
    ans += std::cmp::max(a[i as usize] - 10, 0);
  }
  println!("{}", ans);
}
