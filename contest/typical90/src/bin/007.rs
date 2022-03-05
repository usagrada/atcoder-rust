use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n],
    q: usize,
    b: [usize; q],
  }
  a.sort();
  for bj in b {
    let ans = {
      let index = {
        if bj <= a[0] {
          0
        } else if a[a.len() - 1] <= bj {
          a.len() - 1
        } else {
          nibutan(&a, bj)
        }
      };
      (bj as i32 - a[index] as i32).abs()
    };
    println!("{}", ans);
  }
}

fn nibutan(a: &Vec<usize>, bj: usize) -> usize {
  let mut left = 0;
  let mut right = a.len() - 1;
  while left + 1 < right {
    let mid = (left + right) / 2;
    if a[mid] <= bj {
      left = mid;
    } else {
      right = mid;
    }
  }
  if a[left] <= bj {
    if bj - a[left] <= a[right] - bj {
      left
    } else {
      right
    }
  } else {
    left
  }
}
