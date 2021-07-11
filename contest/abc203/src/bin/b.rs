use proconio::input;

fn main() {
  input!{
    n: usize,
    k: usize
  }
  let mut ans = 0;
  for i in 1..=n {
    ans += i*100*k;
  }
  for j in 1..=k {
    ans += j*n;
  }

  println!("{}", ans);
}
