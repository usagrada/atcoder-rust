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
  let mut note = vec![vec![false; n]; n];

  for i in 0..n {
    let (a, b) = e[i];
    note[a-1][b-1]= true;
  }

  for i in 0..n {
    let mut visitable = vec![false; n];
    visitable[i]=true;
    search(i, n, &note, &mut visitable);
    for j in 0..n{
      ans += if visitable[j] {1}else{0}
    }
  }
  
  println!("{}", ans);
}

fn search(index: usize, n:usize, note: & Vec<Vec<bool>>, visitable: &mut Vec<bool>){
  for i in 0..n{
    if note[index][i] {
      if !visitable[i] {
        visitable[i] = true; 
        search(i, n, note, visitable);
      }
    }
  }
}