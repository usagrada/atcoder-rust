use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n],
  }
  let mut v = vec![0; 200];
  for i in 0..n {
    a[i] %= 200;
    v[a[i]] += 1;
  }
  let mut ans = 0;

  for i in 0..200 {
    ans += if v[i] > 1 { nc2(v[i]) } else { 0 };
  }
  println!("{}", ans);
}

fn nc2(n: usize)-> usize {
  n * (n - 1) / 2
}
