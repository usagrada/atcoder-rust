use proconio::input;
fn main() {
  input! {
    n: usize,
    cp: [(usize, usize); n],
    q: usize,
    lr: [(usize, usize); q]
  };
  let mut all_data = vec![0_usize; n + 1];
  let mut one_data = vec![0_usize; n + 1];
  for i in 0..n {
    let (c, p) = cp[i];
    if c == 1 {
      one_data[i+1] = one_data[i] + p;
      all_data[i+1] = all_data[i] + p;
    } else {
      one_data[i+1] = one_data[i];
      all_data[i+1] = all_data[i] + p;
    }
  }

  for (l, r) in lr {
    let ans1 = one_data[r] - one_data[l-1];
    let ans2 = all_data[r] - all_data[l-1] - ans1;
    
    println!("{} {}", ans1, ans2);
  }
}
