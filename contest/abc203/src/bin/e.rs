use proconio::input;

fn main() {
  input!{
    n: usize,
    m: usize,
    mut z: [(usize, usize); m]
  }
  let mut ans = 0;
  z.sort_by_key(|&a| a.1);

  let mut min_index = n;

  for i in 0..m {
    if z[i].1 > n {
      min_index = i;
      break;
    }
  }
  if min_index == n {
    // n を上回る要素が存在しなかった
    let mut nowi = n;
    for i in 0..1 {
      if z[i].1 == nowi - 1 && (i==0 || z[i-1].1 != z[i].1) && (i == m-1 || z[i].1 != z[i+1].1) {
        nowi -= 1;
        ans += 1;
      }else {
        break;
      }
    }
  }
  
  else {
    println!("pattern 2");
    // n を上回る要素が存在しなかった
    let mut nowi = n;
    for i in (min_index-1)..=0 {
      if z[i].1 == nowi - 1 && (i==0 || z[i-1].0 != z[i].0) && (i == m-1 || z[i].0 != z[i+1].0) {
        nowi -= 1;
        ans += 1;
      }else {
        break;
      }
    }
    nowi = n;
    for i in min_index..m {
      if z[i].1 == nowi - 1 && (i==0 || z[i-1].0 != z[i].0) && (i == m-1 || z[i].0 != z[i+1].0) {
        nowi += 1;
        ans += 1;
      }else {
        break;
      }
    }
  }
  println!("{}", ans);
}
