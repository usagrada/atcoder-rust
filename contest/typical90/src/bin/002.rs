use proconio::input;
use std::string::String;

fn main() {
  input! {
    n: usize,
  }

  if n % 2 == 1 {
    return;
  }

  solve(n);
}

fn solve(n: usize) {
  let left = "(";
  let right = ")";

  for bit in 0..(1 << n) {
    let mut string = String::new();
    for i in 0..n {
      if (bit & 1 << (n-i-1)) != 0 {
        // string.add(left);
        string += right;
      } else {
        string += left;
      }
    }
    if check(&string) {
      println!("{}", string);
    }
  }
}

fn check(string: &String) -> bool {
  let mut cnt = 0;
  for letter in string.chars() {
    if letter == '(' {
      cnt += 1;
    }else {
      cnt -= 1;
    }
    if cnt < 0 {
      return false;
    }
  }
  if cnt != 0 {
    return false;
  }

  return true;
}
