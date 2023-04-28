use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        m: usize,
        width: usize,
        height: usize,
        b: usize,
        depth: usize,
        hwdafg: [(usize, usize, usize, usize, char, char); m],
    }
    // h: height, w: width, d: depth, a: 個数, f: 回転, g: 上に積むことが可能か
    // output: p r x y z
    // p: 荷物の種類
    // r: 回転の向き
    // x y z: ブロックの位置

    let mut container = Container::new(width, height, depth);
    container.fill(0, 0, 0, b, b, depth);
    container.fill(0, height - b, 0, b, b, depth);
    container.fill(width - b, 0, 0, b, b, depth);
    container.fill(width - b, height - b, 0, b, b, depth);

    let mut floor = vec![vec![]; depth];
    let mut priority_queue = BinaryHeap::new();

    for i in 0..m {
        let (mut h, mut w, mut d, a, f, g) = hwdafg[i];
        let mut r = 0;
        if f == 'Y' {
            // 回転して高さを減らせる場合減らす
            if h < d && h < w {
                (h, d) = (d, h);
                r = 4;
            } else if w < d && w < h {
                (w, d) = (d, w);
                r = 2;
            }
        }
        priority_queue.push((a, i, h, w, d, r, g));

        for ai in 0..a {
            // container.fill((b + 1) * ai, 0, 0, w, h, d);
            loop {
                let x = rand::Rng::gen_range(&mut rand::thread_rng(), 0, width);
                let y = rand::Rng::gen_range(&mut rand::thread_rng(), 0, height);
                for z in 0..d {
                    let flag = container.fill_check(x, y, z, w, h, d);
                    if flag {
                        continue;
                    }
                    container.fill(x, y, z, w, h, d);
                    floor[z + d].push((x, y, w, h));
                    break;
                }
            }
        }
    }
}

struct Container {
    width: usize,
    height: usize,
    depth: usize,
    filled: Vec<Vec<Vec<bool>>>,
}

impl Container {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let filled = vec![vec![vec![false; depth]; height]; width];
        Self {
            width,
            height,
            depth,
            filled,
        }
    }

    fn fill(&mut self, x: usize, y: usize, z: usize, width: usize, height: usize, depth: usize) {
        for i in x..x + width {
            for j in y..y + height {
                for k in z..z + depth {
                    self.filled[i][j][k] = true;
                }
            }
        }
    }
    fn fill_check(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> bool {
        for i in x..x + width {
            for j in y..y + height {
                for k in z..z + depth {
                    if self.filled[i][j][k] {
                        return true;
                    }
                }
            }
        }
        false
    }
}
