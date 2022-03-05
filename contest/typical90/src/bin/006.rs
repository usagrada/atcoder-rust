use proconio::input;
use std::collections::VecDeque;

fn main() {
  input! {
    n: usize,
    k: usize,
    mut s: String,
  }

  let mut left = 0;
  let mut right = n - k + 1;

  let mut cnt_alph = vec![VecDeque::new(); 26];
  for i in 0..=right {
    let c = s.chars().nth(0).unwrap();
    cnt_alph[c as usize - 'a' as usize].push_back(i);
  }
  let mut ans = "".to_string();

  for cnt in 0..k {
    let mut idx = 0;
    for alph in 0..26 {
      if cnt_alph[alph].len() > 0 {
        idx = cnt_alph[alph].pop_front().unwrap();
        ans.push(('a' as u8 + alph as u8) as char);
        break;
      }
    }
    let ol = left;
    let or = right;
    left = idx + 1;
    right = std::cmp::min(n-1, n - (k - cnt) + 1);
    let sc = s.chars();
    for index in ol..=idx {
      let sc = sc.nth(index).unwrap()
      cnt_alph[c as usize - 'a' as usize].pop_front();
    }
    for index in or+1..=right {
      let c = s.chars().nth(index).unwrap();
      cnt_alph[c as usize - 'a' as usize].push_back(index);
    }
    // println!("{left} {right}");
  }

  println!("{}", ans);
}
