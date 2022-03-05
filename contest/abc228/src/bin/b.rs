use proconio::input;
fn main() {
  input! {
    n: usize,
    x: usize,
    a: [usize; n]
  }
  let mut next = x - 1;
  let mut check = vec![false; n];
  let mut cnt = 0;

  loop {
    if check[next] {
      break;
    } else {
      check[next] = true;
      cnt += 1;
      next = a[next] - 1;
    }
  }

  // ans
  println!("{}", cnt);
}
