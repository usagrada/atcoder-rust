use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut c: [(Usize1, Usize1); m]
  }

  // make graph
  let mut uf = UnionFind::new(n + 1);
  let mut edge = vec![vec![]; n + 1];
  for (a, b) in c {
    edge[a].push(b);
  }

  let mut ans_vec = vec![0];
  let mut ans = 0;
  for node in (1..n).rev() {
    // add Node node
    ans += 1;
    for &e in &edge[node] {
      if uf.same(node, e) {
        continue;
      }
      uf.unite(node, e);
      ans -= 1;
    }
    // ans_vec[node] = ans;
    ans_vec.push(ans);
  }

  for i in (0..ans_vec.len()).rev() {
    println!("{}", ans_vec[i]);
  }
}

#[derive(Debug, Clone)]
struct UnionFind {
  n: usize,
  parents: Vec<usize>,
}

impl UnionFind {
  pub fn new(n: usize) -> Self {
    Self {
      n: n,
      // parents: parents
      parents: (0..n).collect(),
    }
  }

  pub fn add(&mut self, index: usize) -> &Self {
    self.n += 1;
    self.parents.push(index);
    self
  }

  pub fn root(&mut self, x: usize) -> usize {
    if self.parents[x] == x {
      x
    } else {
      self.parents[x] = self.root(self.parents[x]);
      self.parents[x]
    }
  }

  pub fn unite(&mut self, x: usize, y: usize) {
    let rx = self.root(x);
    let ry = self.root(y);
    if rx == ry {
      return;
    }
    self.parents[ry] = rx;
    // self
  }

  pub fn same(&mut self, x: usize, y: usize) -> bool {
    let rx = self.root(x);
    let ry = self.root(y);
    rx == ry
  }
}
