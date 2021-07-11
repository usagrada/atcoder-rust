use std::collections::HashMap;
use proconio::input;

fn main() {
  input!{
    n: usize,
    a: [usize; n],
    b: [usize; n],
    c: [usize; n],
  }
  let mut map1 = HashMap::new();
  let mut map2 = HashMap::new();
  for i in 0..n {
    let check = map1.contains_key(&a[i]);
    if check {
      *map1.get_mut(&a[i]).unwrap() += 1;
    }else{
      map1.insert(&a[i], 1);
    }
    let check = map2.contains_key(&b[c[i]-1]);
    if check {
      *map2.get_mut(&b[c[i]-1]).unwrap() += 1;
    }else{
      map2.insert(&b[c[i]-1], 1);
    }

  }
  let mut ans: u64 = 0;
  for i in 0..=n {
    if map1.contains_key(&i) && map2.contains_key(&i){
      ans += map1[&i] * map2[&i];
    };
  }

  println!("{}", ans);

}