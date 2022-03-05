use std::f64::consts::PI;
use proconio::input;
use libm::{sin, cos};
fn main() {
  input! {
    t: usize,
    lxy: (usize, usize, usize),
    q: usize,
    e: [usize; q]
  }
  let (l, x, y) = lxy;
  for eq in e {
    let zou = Zahyo {x: x as f64, y: y as f64, z: 0.0};
    let theta = 2.0 * PI * eq as f64 / t as f64 - PI / 2.0;
    let l2 = l as f64 / 2.0;
    let zahyo = Zahyo {x: 0.0, y: -cos(theta) * l2, z: l2 * (1.0 + sin(theta))};
    let ans = calc(zou, zahyo);
    println!("{}", ans);
  }
}

struct Zahyo {
  x: f64,
  y: f64,
  z: f64,
}

fn calc(w0: Zahyo, w1: Zahyo) -> f64 {
  let x = w1.x - w0.x;
  let y = w1.y - w0.y;
  let z = w1.z - w0.z;
  let r = (x * x + y * y).sqrt();
  let theta = (z / r).atan();
  theta * 180.0 / PI
}
