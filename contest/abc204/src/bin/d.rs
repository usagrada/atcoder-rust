use proconio::input;

fn main() {
  input! {
    n: usize,
    mut t: [usize; n],
  }

  let mut sum = 0;
  for i in &t {
    sum += i;
  }

  let mut dp = vec![vec![false; sum + 1]; n + 1];
  dp[0][0] = true;

  for i in 0..n {
    for j in 0..=sum {
      if j >= t[i] && dp[i][j - t[i]] {
        dp[i + 1][j] = true;
      } else if dp[i][j] {
        dp[i + 1][j] = true;
      }
    }
  }
  for i in ((sum + 1) / 2)..=sum {
    if dp[n][i] {
      println!("{}", i);
      break;
    }
  }
}
