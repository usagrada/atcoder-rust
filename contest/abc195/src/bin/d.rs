use proconio::input;
use std::cmp;

fn main() {
  input! {
    n: i32,
    m: i32,
    q: i32,
    mut z: [(i32, i32); n],
    x: [i32; m],
    queries: [(i32, i32); q],
  }
  let MAX_M = 50;
  let mut note: Vec<i32> = vec![0; MAX_M+1];
  // z.sort();
  z.sort_by_key(|k| k.1);
  z.reverse();
  // for &(weight, value) in &z {
  //   println!("weight:{} value: {}", weight, value);
  // }
  // return;


  for &(l, r) in &queries {
    /*
    1, 2, …, l-1, r+1, r+2, …, n
    が使える 
    */
    let mut note = vec![true; m as usize];
    let mut ans = 0;
    for &(w, v) in &z{
      let mut index = -1;
      let mut min_cap = 1_000_000;
      for i in 0..m{
        if i >= l-1 && i <= r-1 {
          continue;
        }
        if note[i as usize] && x[i as usize]>=w {
          min_cap = cmp::min(min_cap, x[i as usize]);
          if min_cap == x[i as usize] {
            index = i;
          }
        }
      }
      if index >= 0 {
        ans += v;
        note[index as usize] = false;
      }
    
    }
    // println!("x: {:?}", note);
    println!("{}", ans);
    
  }


  
}
