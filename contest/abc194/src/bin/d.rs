use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  
  accode(&n);
  return;

  if n == 2 {
    let ans = 2.0;
    println!("{:.6}", ans);  
    return;
  }
  
  let num = n;
  let n = n as f64;
  let mut ans: f64 = n*(n-1.0f64) - n*(n-1.0f64)*(n-2.0f64)/4.0f64;

  for i in 1..(num-1) {
    let id = i as f64;
    ans -= id * ((n-1.0f64)/n).powf(id);
    ans += id * (n-1.0f64)*(n-2.0f64) / (n-2.0f64) * ((n-2.0f64)/n).powf(id);
    // println!("{}, {:.8}", i, ans);
  }
 
  println!("{:.8}", ans);

}


fn accode(&n: &usize){
  let mut ans = 0.0;
  let num = n as f64;
  for i in 1..n {
    let id = i as f64;
    ans += num/id;
  }

  println!("{:.8}", ans);
}