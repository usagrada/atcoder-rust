use itertools::Itertools;
use proconio::input;
use superslice::*;
fn main() {
  input! {
    n: usize,
    q: usize,
    mut a: [u64; n],
    k: [u64; q]
  }

  // 解説AC
  let mut c = a.clone();
  for i in 0..n {
    c[i] -= i as u64 + 1;
  }

  for ki in k {
    // 大きい
    if c[n - 1] < ki {
      println!("{}", ki + n as u64);
    } else {
      // nibutan
      // let memorize = nibutan(&n, &c, &ki);
      let c = c
        .iter()
        .enumerate()
        .map(|(i, &x)| x - i as u64)
        .collect_vec();
      let memorize = c.lower_bound(&ki) - 1;
      // let mut newki = ki;
      // println!("memorize: {}", memorize);

      let newki = a[memorize] - 1 - (c[memorize] - ki);
      println!("{}", newki);
    }
  }

  return;
  // MyCode
  a.sort();
  for ki in k {
    // 大きい
    if a[n - 1] < ki {
      println!("{}", ki + n as u64);
    } else {
      // nibutan
      let memorize = nibutan(&n, &a, &ki);
      let mut newki = ki;

      newki += memorize as u64;
      let mut tmp = memorize;
      if memorize == n {
        newki += 1;
      }
      loop {
        if tmp == n {
          break;
        }
        if a[tmp] <= newki {
          newki += 1;
        } else {
          break;
        }
        tmp += 1;
      }
      // println!("memorize: {}, tmp: {}", memorize, tmp);
      println!("{}", newki);
    }
  }
}

fn nibutan(n: &usize, a: &Vec<u64>, ki: &u64) -> usize {
  let ki = *ki;

  let mut ng = -1 as isize;
  let mut ok = *n as isize;

  while ok - ng > 1 {
    let center = ((ok + ng) / 2) as usize;
    if a[center] >= ki {
      ok = center as isize;
    } else {
      ng = center as isize;
    }
  }
  ok as usize
  // right
}
