use proconio::input;

fn ans(flag: bool) {
  if flag {
    println!("Yes");
  } else {
    println!("No");
  }
}

fn main() {
  input! {
    s: usize,
    t: usize,
    x: usize
  }
  if s < t {
    ans(s <= x && x < t);
  } else {
    ans(x < t || s <= x)
  }
}
