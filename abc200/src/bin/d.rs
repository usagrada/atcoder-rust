use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n],
  }

  let mut flag = false;
  let mut dp = vec![vec![0; 200]; n + 1];
  for i in 0..n {
    a[i] %= 200;
  }

  for i in 0..n {
    for j in 0..200 {
      if dp[i][j] > 0 {
        dp[i + 1][j] += 1;
        dp[i + 1][(j + a[i]) % 200] += 1;
      }
    }
    // i番目だけ
    dp[i + 1][a[i]] += 1;
  }

  let mut num = 0;
  'outer: for i in 0..=n {
    for j in 0..200 {
      if dp[i][j] > 1 {
        flag = true;
        num = j;
        break 'outer;
      }
    }
  }

  if flag {
    println!("Yes");
    solve(num, &a, &dp);
  } else {
    println!("No");
  }
}

fn solve(num: usize, a: &Vec<usize>, dp: &Vec<Vec<usize>>) {
  // 総和を200で割った余りがnumに等しい組み合わせを出力
  let mut v1 = Vec::new();
  let mut v2 = Vec::new();

  let mut cnt = 0;
  for i in 0..a.len() {
    if dp[i][num] < dp[i + 1][num] {
      if cnt == 0 {
        v1.push(i);
        dfs((num + 200 - a[i]) % 200, i, &mut v1, &a, &dp, true);
        cnt += 1;
        if dp[i][num] + 1 < dp[i + 1][num] {
          v2.push(i);
          dfs((num + 200 - a[i]) % 200, i, &mut v2, &a, &dp, false);
          break;
        }
      } else if cnt == 1 {
        v2.push(i);
        dfs((num + 200 - a[i]) % 200, i, &mut v2, &a, &dp, false);
        break;
      }
    }
  }

  v1.sort();
  v2.sort();
  print!("{} ", v1.len());
  for i in &v1 {
    print!("{} ", i + 1);
  }
  println!("");
  print!("{} ", v2.len());
  for i in &v2 {
    print!("{} ", i + 1);
  }
  println!("");
}
fn dfs(
  num: usize,
  index: usize,
  vec: &mut Vec<usize>,
  a: &Vec<usize>,
  dp: &Vec<Vec<usize>>,
  flag: bool,
) {
  if flag && num == 0 {
    return;
  }
  if index == 0 && num == a[index] {
    // vec.push(index);
    return;
  }
  for i in 0..index {
    if dp[i][num] < dp[i + 1][num] {
      vec.push(i);
      dfs((num + 200 - a[i]) % 200, i, vec, &a, &dp, flag);
      break;
    }
  }
}
