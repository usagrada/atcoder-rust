use proconio::input;
fn main() {
  input!{
    h: usize,
    w: usize,
    area: [[usize; w]; h],
  }

  let mut sum_row = vec![0; h];
  let mut sum_col = vec![0; w];
  for i in 0..h {
    for j in 0..w {
      sum_row[i] += area[i][j];
      sum_col[j] += area[i][j];
    }
  }
  
  for i in 0..h {
    for j in 0..w {
      print!("{} ", sum_row[i]+ sum_col[j] - area[i][j]);
    }
    println!("");
  }
}
