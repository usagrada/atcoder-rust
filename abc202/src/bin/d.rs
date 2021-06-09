use proconio::input;

fn dfs(s: &mut String, a: usize, b: usize, res: u64, note:&Vec<Vec<u64>>) {
  
  if a == 0 && b == 0 {
    // println!("finished!!");
    return;
  }
  else if a == 0 {
    *s += "b";
    dfs(s, a, b - 1, res, note);
    return;
  }
  else if b == 0 {
    *s += "a";
    dfs(s, a - 1, b, res, note);
    return;
  }

  let num: u64 = note[a-1][b];
  // println!("num: {}", num);
  // println!("{} {} {} {}", s, a, b, res);
  if num >= res {
    *s += "a";
    dfs(s, a - 1, b, res, note);
  } else {
    *s += "b";
    dfs(s, a, b - 1, res - num, note);
  }
}

fn main() {
  input! {
    a: usize,
    b: usize,
    k: u64,
  }

  let mut ans = "".to_string();
  let mut note: Vec<Vec<u64>> = vec![vec![0; 40]; 40];

  note[0][1] = 1 as u64;
  note[1][0] = 1 as u64;

  dp(&mut note);
  dfs(&mut ans, a, b, k, &note);
  //   let mut re = k;
  //   let mut rea = a;
  //   let mut reb = b;

  //   for index in 0..a + b {
  //     if note[index][b] > re {
  //     }
  //   }
  println!("{}", ans);
}

fn dp(note: &mut Vec<Vec<u64>>) {
  for i in 0..30 {
    for j in 0..30 {
      note[i + 1][j] += note[i][j];
      note[i][j + 1] += note[i][j];
    }
  }
}


