use proconio::input;

fn main() {
  input!{
    n: usize,
    k: usize,
    mut a: [[usize; n]; n]
  }
  let mut ans = 1_000_000_000 + 1;

  ans = std::cmp::min(calc(&a, 0, 0, k), ans);
  for i in 0..=(n-k) {
    let mut count_low = 0;
    let mut count_high = 0;

    for j in 0..k {
      if a[i][j] > ans {
        count_low+=1;
      }else {
        count_high+=1; 
      }
    }
    // for j in 0..=(n-k) {
      // println!("{}", ans);
      // ans = std::cmp::min(calc(&a, i, j, k), ans)

    // }
  }
  
  println!("{}", ans);
}

fn calc(a: &Vec<Vec<usize>>, i: usize, j: usize, k: usize)->usize{
  let mut d = Vec::new();
  for row in 0..k {
    for col in 0..k {
      d.push(a[i+row][j + col]);
    }
  }
  d.sort();
  // println!("d {:?}", d);
  d[k*k-k*k/2-1]
}
