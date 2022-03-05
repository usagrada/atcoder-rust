use proconio::input;
const MOD: u64 = 998244353;

fn mod_pow(x: u64, y: u64, m: u64) -> u64 {
  if y == 0 {
    1
  } else {
    let h = mod_pow(x, y / 2, m);
    let res = (h * h) % m;
    if y % 2 == 0 {
      res
    } else {
      (res * x) % m
    }
  }
}

fn main() {
  input! {
    n: u64,
    k: u64,
    m: u64,
  }

  // ans = m ^ (k ^ n)
  let k = k % (MOD - 1);
  let m = m % MOD;
  let p_num = mod_pow(k, n, MOD - 1);
  let ans = mod_pow(m % MOD, p_num, MOD);
  println!("{}", ans % MOD);
}
