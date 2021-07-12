use proconio::input;
fn main() {
  input! {
    n: usize,
    e: [(usize, usize); n-1],
  }

  let mut edge = vec![vec![]; n];
  for i in 0..n - 1 {
    edge[e[i].0 - 1].push(e[i].1 - 1);
    edge[e[i].1 - 1].push(e[i].0 - 1);
  }

  let mut dis = vec![0; n];
  bfs(&edge, &mut dis, 0);
  // println!("dist: {:?}", dis);

  let mut maxi = 0;
  let mut maxv = 0;
  for i in 1..n {
    if dis[i] > maxv {
      maxi = i;
      maxv = dis[i];
    }
  }

  let mut dis2 = vec![0; n];
  bfs(&edge, &mut dis2, maxi);
  // println!("dist2: {:?}", dis2);

  maxv = 0;
  for i in 0..n {
    if dis2[i] > maxv {
      maxv = dis2[i];
    }
  }
  println!("{}", maxv+1);
}
fn bfs(edge: &Vec<Vec<usize>>, dis: &mut Vec<usize>, start: usize) {
  let n = dis.len();
  let mut visited = vec![false; n];
  visited[start] = true;

  // let mut flag = true;
  let mut arr = vec![start];
  while arr.len()>0 {
    // flag = false;
    let index = arr.pop().unwrap_or(n);
    if index == n {
      break;
    }
    
    for i in &edge[index] {
      if visited[*i] {
        continue;
      }
      dis[*i] = dis[index] + 1;
      visited[*i] = true;
      arr.push(*i);
      // flag = true;
    }
  }
}
