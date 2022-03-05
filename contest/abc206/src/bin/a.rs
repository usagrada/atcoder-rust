use proconio::input;
fn main() {
  input! {
    n: usize,
  }
  let minnum: f32 = 206.0;
  let maxnum: f32 = 207.0;
  if minnum > (n as f32 * 1.08) {
    println!("Yay!");
  } else if maxnum <= (n as f32 * 1.08) {
    println!(":(");
  } else {
    println!("so-so");
  }
}
