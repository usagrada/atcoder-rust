use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize,
  }
  let milk = a + b;

  if milk >= 15 && b>=8 {
    println!("1");
  }else if milk >= 10 && b >= 3{
    println!("2");
  }else if milk >= 3 {
    println!("3");
  }else{
    println!("4");
  }

}
