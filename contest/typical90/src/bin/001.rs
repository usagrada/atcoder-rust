use proconio::input;
fn main() {
  input! {
    n: usize,
    l: usize,
    k: usize,
    a: [usize; n],
  }

  let mut ans = 0_usize;
  let max: usize = l / (k + 1);

  let mut i = 0;
  let mut left = 0;
  let mut right = max;
  // for i in 1..max {
  while left <= right {
    i = left + (right - left) / 2;
    // println!("l:{}, r:{}, i:{}", left ,right, i);
    if check(&a, i, k, l) {
      ans = i;
      left = i+1;
    } else {
      right = i-1;
    };
  }
  
  println!("{}", ans);
}

fn check(a: &Vec<usize>, num: usize, k: usize, l: usize) -> bool {
  let mut index = 0;
  let mut cnt = 0;
  for i in 0..a.len() {
    if a[i] - index >= num {
      index = a[i];
      cnt += 1;
    }
  }

  if l - index >= num {
    cnt += 1;
  }

  if cnt <= k {
    false
  } else {
    true
  }
}
