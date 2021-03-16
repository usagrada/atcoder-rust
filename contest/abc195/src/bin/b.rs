use proconio::input;
use std::cmp;

fn main() {
  input!{
    a: i32, // g
    b: i32, // g
    w: i32, // kg
  }
  // let mut flag = true;
  let w = w * 1000;

  // let dif = b - a;
  let max = w/a + 1;
  let min = w/b;

  let mut ans_min = 2_000_001;
  let mut ans_max = -1;
  for i in min..max {
    if a * i <= w && w <= b*i {
      ans_min = cmp::min(ans_min, i);
      ans_max = cmp::max(ans_max, i);
    }
  }

  if ans_min > 2_000_000 || ans_max < 0 {
    println!("UNSATISFIABLE");
    // println!("{} {} {}", a, b, w);
  }else{
    println!("{} {}", ans_min, ans_max);
  }
}
