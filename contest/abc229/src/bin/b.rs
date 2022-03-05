use proconio::input;
fn main() {
  input! {
    a: String,
    b: String
  }
  let al = a.len();
  let bl = b.len();
  let l = if al < bl { al } else {bl};

  for i in 0..l {
    let ai: usize = a[i].parse().unwrap();
    let bi: usize = b[i].clone().parse().unwrap();
  }

  ans(true);
}

fn ans(flag: bool) {
  if flag {
    println!("Easy")
  } else {
    println!("Hard")
  }
}
