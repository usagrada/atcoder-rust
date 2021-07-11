use proconio::input;

fn main() {
  input!{
    n: usize,
    k: usize,
    mut a: [[usize; n]; n]
  }
  let ans = 1_000_000_000 + 1;

  for i in 0..(n-k) {
    for j in 0..(n-k) {
      ans = std::cmp::min(calc(a, i, j, k), ans)
    }
  }
  
  println!("{}", ans);
}

fn calc(a: &vec![vec![usize]], i: usize, j: usize, k: usize)->usize{
  let mut d = Vec::new();
  for row in 0..k {
    for col in 0..k {
      d.push(a[i+row][j + col]);
    }
  }
  d.sort();
  d[k*k/2]
}
