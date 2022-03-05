use proconio::input;

const N: usize = 1_048_576; // 2^20

fn main() {
  input! {
    num: usize,
    querys: [(usize, usize); num]
  }

  // database
  let mut db: Vec<isize> = vec![-1; N];
  let mut hash: Vec<usize> = (0..N).collect();
  for query in querys {
    // query = (t, x)
    // println!("{:?}", query);
    
    if query.0 == 1 {
      let mut h = hash[query.1 % N];
      while db[h] != -1 {
        h += 1;
        if h == N {
          h = 0;
        }
      }
      hash[query.1 % N] = h;
      db[h] = query.1 as isize;
    } else if query.0 == 2 {
      let h = query.1 % N;
      println!("{}", db[h]);
    }
  }
}
