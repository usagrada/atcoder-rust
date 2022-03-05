use proconio::input;
use std::collections::HashMap;
fn main() {
  input! {
    n: usize,
    a: [usize; n]
  }

  // let mut map = HashMap::new();
  let mut numhash = HashMap::new();
  let mut uf = UF::new(2_00_001);
  for i in 0..n {
    if i > n - i - 1 {
      break;
    } else if a[i] == a[n - i - 1] {
      // pass
    } else {
      *numhash.entry(a[i]).or_insert(0) += 1;
      *numhash.entry(a[n - i - 1]).or_insert(0) += 1;
      uf.union(a[i], a[n - i - 1]);
    }
  }

  let mut usedhash = HashMap::new();
  let mut ans = 0;
  for (key, _value) in numhash.iter() {
    // println!("key: {}, value :{}", key, value);
    let root = uf.find(*key);
    let check = usedhash.entry(root).or_insert(true);
    if *check {
      *check = false;
      // let mems = uf.members(root);
      // ans += mems.len() - 1;
      ans += uf.size(root) - 1;
    }
  }

  println!("{}", ans);
}

/// UnionFind
/// * `n`: 大きさ
/// * `sizes`: グループのサイズを確保する場所
/// * `parents`: 親
struct UF {
  n: usize,
  sizes: Vec<usize>,
  parents: Vec<usize>,
}

impl UF {
  /// Create UnionFind
  /// # Example
  /// ```
  /// let mut uf = UF::new(100);
  /// ```
  fn new(n: usize) -> Self {
    UF {
      n: n,
      sizes: vec![1; n],
      parents: (0..n).collect(),
    }
  }

  /// find root
  fn find(&mut self, num: usize) -> usize {
    let par = self.parents[num];
    if par == num {
      num
    } else {
      let par = self.find(par);
      self.parents[num] = par;
      par
    }
  }

  /// unite leaf
  fn union(&mut self, num1: usize, num2: usize) {
    let mut par1 = self.find(num1);
    let mut par2 = self.find(num2);

    if par1 == par2 {
      return;
    }
    if self.sizes[par1] < self.sizes[par2] {
      std::mem::swap(&mut par1, &mut par2);
    }

    self.sizes[par1] += self.sizes[par2];
    self.parents[par2] = par1;
  }

  /// check whether it is the same route
  /// 
  /// # Example
  /// ```
  /// uf.same(1, 2);
  /// ```
  fn same(&mut self, num1: usize, num2: usize) -> bool {
    self.find(num1) == self.find(num2)
  }

  /// return same root members
  fn members(&mut self, num: usize) -> Vec<usize> {
    let par = self.find(num);
    (0..self.n).filter(|&x| self.find(x) == par).collect()
  }

  /// size of the same root members
  fn size(&mut self, num: usize) -> usize {
    let root = self.find(num);
    self.sizes[root]
  }
}
