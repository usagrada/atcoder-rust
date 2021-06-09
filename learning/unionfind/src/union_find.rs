#[derive(Debug)]
pub struct UnionFind {
    n: usize,
    parents: Vec<usize>,
}

impl UnionFind {
    pub fn new(n:usize) -> Self {
        // let mut parents = vec![0; n];
        // for i in 0..n {
        //     parents[i] = i;
        // }
        UnionFind {
            n: n,
            // parents: parents
            parents: (0..n).collect()
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        }else {
            self.parents[x] = self.root(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x:usize, y: usize) -> &Self {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return self;
        }
        self.parents[ry] = rx;
        self
    }

    pub fn same(&mut self, x:usize, y:usize)-> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        rx == ry
    }
}