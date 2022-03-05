use proconio::input;
const MOD: usize = 1_000_000_007;
const atcoder = "atcoder";

fn main() {
  input! {
    n: usize,
    s: String,
  }
  let len = atcoder.len();
  let mut ans = 0;
  let ans += calc(&s);
  println!("{}", dp[n][6] % MOD);
}

fn calc(s: &str) -> usize {
  let cnt = [0; atcoder.len()];
  let mut ans = 0;
  for i in 0..s.len() {
    for ch in atcoder.chars(){
      solve(&s[i..], ch, &mut cnt);
    }
  }
  
  0
}

fn solve(&s: &str, ch: char, cnt: [usize; 6]) -> usize{
  0
}