use proconio::input;
fn main() {
    input! {
      n: usize,
      a: [usize; n]
    }

    let mut nums = [0; 2001];
    for i in 0..n{
      nums[a[i]] += 1;
    }

    for i in 0..2001 {
      if nums[i] == 0 {
        println!("{}", i);
        return;
      }
    }

}
