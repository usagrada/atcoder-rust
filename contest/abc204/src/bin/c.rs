use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    e: [(usize, usize); m]
  }
  let mut ans = 0;
  if m == 0 {
    println!("{}", n);
    return;
  }
  let mut note = vec![vec![]; n];

  for i in 0..m {
    let (a, b) = e[i];
    note[a - 1].push(b - 1);
  }

  for i in 0..n {
    let mut visitable = vec![false; n];
    visitable[i] = true;
    search(i, n, &note, &mut visitable);
    for j in 0..n {
      ans += if visitable[j] { 1 } else { 0 }
    }
  }
  println!("{}", ans);
}

fn search(index: usize, n: usize, note: &Vec<Vec<usize>>, visitable: &mut Vec<bool>) {
  for i in note[index].iter() {
    // if {
    if !visitable[*i] {
      visitable[*i] = true;
      search(*i, n, note, visitable);
    }
  }
  // }
}
