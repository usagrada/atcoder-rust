use proconio::input;
const MOD: u128 = 998244353;

fn mod_pow(x: u128, y: u128, m: u128) -> u128 {
  if x == 0 {
    0
  } else if y == 0 {
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
    n: u128,
    k: u128,
    m: u128,
  }

  // ans = m ^ (k ^ n)
  // let k = k % (MOD-1);
  // let m = m % MOD;
  let p_num = mod_pow(k, n, MOD - 1);
  let ans = mod_pow(m % MOD, p_num, MOD);
  println!("{}", ans % MOD);
}
