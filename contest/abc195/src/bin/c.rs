use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: i64,
  }
  let i3: i64 = 999;
  let i6: i64 = 999_999;
  let i9: i64 = 999_999_999;
  let i12: i64 = 999_999_999_999;
  let i15: i64 = 999_999_999_999_999;
  let i18: i64 = 999_999_999_999_999_999;
  
  let mut ans = 0;

  if n > i15 {
    let m = cmp::min(n, i18);
    ans += (m-i15)*5;
  }
  if n > i12{
    let m = cmp::min(n, i15);
    ans += (m-i12)*4;
  }
  if n > i9 {
    let m = cmp::min(n, i12);
    ans += (m-i9)*3;
  }
  if n > i6 {
    let m = cmp::min(n, i9);
    ans += (m-i6)*2;
  }
  if n > i3 {
    let m = cmp::min(n, i6);
    ans += m-i3;
  }
  
  println!("{}", ans);
}
