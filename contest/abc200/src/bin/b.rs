use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
  }
  let mut num = n;
  for _ in 0..k {
    if num % 200 == 0 {
      num /= 200;
    } else {
      num *= 1000;
      num += 200;
    }
  }
  println!("{}", num);
}
