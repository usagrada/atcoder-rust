use proconio::input;

fn ans(flag: bool) {
  if flag {
    println!("Yes");
  } else {
    println!("No");
  }
}

fn main() {
  input! {
    n: usize,
    k: usize,
    p: [[usize; 3]; n]
  }
  let sums: Vec<usize> = p
    .iter()
    .map(|points| points.iter().fold(0, |s, i| s + i))
    .collect();
  let mut order = sums.clone();
  // order は 0-index
  order.sort();
  order.reverse();
  let standard = order[k - 1];
  // println!("{:?}", order);
  // println!("{:?}", sums);
  for i in 0..n {
    if sums[i] >= standard {
      ans(true);
    } else {
      // ans(false);
      // println!("{}, {}, {}, {}", sums[i], sums[i] + 300, standard, sums[i] + 300 >= standard);
      ans(sums[i] + 300 >= standard);
    }
  }
}
