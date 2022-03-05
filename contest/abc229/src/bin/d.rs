use proconio::input;
fn main() {
  input! {
    s: String,
    k: isize
  }

  let l = s.len();
  let mut cnts = vec![0_isize; l + 1];
  let mut tmp = 0;
  for (i, c) in s.chars().enumerate() {
    match c {
      '.' => {
        tmp += 1;
      }
      _ => {}
    }
    cnts[i + 1] = tmp;
  }

  let mut ans = 0;
  let mut right = 0;
  for left in 0..l {
    // left .. right
    while right < l && cnts[right+1] - cnts[left] <= k {
      right += 1;
    }
    ans = std::cmp::max(ans, right - left);
  }
  println!("{} ", ans);
}
