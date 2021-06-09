use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [i32; n],
  }  

  let mut ans: i64 = 0;
  a.sort();
  let mut num = vec![0; 500];
  // println!("{:?}", num);

  for i in 0..n {
    num[(a[i] + 250) as usize]+=1;
  }

  for i in 0..500 {
    for j in 0..i {
      ans += ((i -j)*(i-j)*num[i]*num[j]) as i64;
      // println!("i: {}, j:{}, ans: {}", i, j, ans);
    }
  }
  println!("{}", ans);

}
