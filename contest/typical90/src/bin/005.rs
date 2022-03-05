use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    b: usize,
    k: usize,
    c: [usize; k],
  }

  // n 桁の整数
  // b の倍数
  // k 個の整数: c[i]

  let keta_b = b.to_string().len();
  if keta_b > n {
    println!("0");
    return;
  }
  else if keta_b == n {
    fn check() -> bool {
      for
    }
    check(&c, b, k);
  }
}
