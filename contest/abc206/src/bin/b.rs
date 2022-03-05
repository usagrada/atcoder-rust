use proconio::input;
fn main() {
  input! {
    n: usize
  }
  let mut sum = 0;
  for i in 1..1_000_000_000 {
    sum += i;
    if n > sum {
      continue;
    }
    // println!("n: {}, sum: {}", n, sum);
    println!("{}", i);
    break;
  }
}
