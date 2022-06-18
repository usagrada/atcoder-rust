# 標準ライブラリ

[厳選！C++ アルゴリズム実装に使える 25 の STL 機能【前編】](https://qiita.com/e869120/items/518297c6816adb67f9a5#3-1-%E7%B5%B6%E5%AF%BE%E5%80%A4-abs)

をRustに代替したもの

## abs

```rust
let a = -1;
let a_abs = (a as isize).abs(); // abs is defined in signed value
println!("{a_abs}");
```

## sin/cos/tan

```rust
let a = 1f64;
let a_sin = a.sin();
println!("{a_sin}");
let a_cos = a.cos();
println!("{a_cos}");
let a_tan = a.tan();
println!("{a_tan}");
```

## 文字列 string

```rust
let s = String::from("string");
println!("{s}");
let mut s = String::new();
s += "hello world";
println!("{s}");
```

## 最小値・最大値 min/max

```rust
let a = 1;
let b = 2;
println!("min: {}", a.min(b));
println!("max: {}", a.max(b));
{
    use std::cmp::{min, max};
    println!("min: {}", min(a, b));
    println!("max: {}", max(a, b));
}
```

```rust
let vec = vec![3,1,4,1,5,9,2];
println!("min: {}", vec.iter().min().unwrap());
println!("max: {}", vec.iter().max().unwrap());
```

## 値の交換 swap

```rust
let (mut a, mut b) = (1, 2);
// after Rust 1.59
(a, b) = (b, a);
// before Rust 1.59
// let (a, b) = (b, a);
println!("{}, {}", a, b);
```

## gcd/lcm

use `num` crate

```rust
let a = 25;
let b = 30;
use num::integer::{gcd, lcm};
let gcd = gcd(a, b);
println!("{gcd}")
```

## 配列を逆順に並び替え reverse

```rust
let mut a = [1,2,3];
a.reverse();
println!("{:?}", a);
```

```rust
let mut vec = vec![1,2,3];
vec.reverse();
println!("{:?}", vec);
```

iterator

```rust
let vec = vec![1,2,3];
for index in (0..vec.len()).rev(){
    println!("{}", vec[index]);
}
```

## ソート sort

```rust
let mut vec = vec![3, 1, 4, 1, 5, 9, 2];
vec.sort();
println!("{:?}", vec);
vec.sort_by_key(|value| -value);
println!("{:?}", vec);
```
