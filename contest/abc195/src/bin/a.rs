use proconio::input;

fn main() {
  input!{
    m: i32,
    h: i32,
  }
  let ans = if h % m == 0 { "Yes" } else { "No" };

  println!("{}", ans);
}
