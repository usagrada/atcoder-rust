use proconio::input;

fn main() {
    input! {
        n: u64
    }
    let mut j = 1_000_000u64;

    let mut ans = std::u64::MAX;
    for i in 0..j {
        let mut tmp = calc(i, j);
        loop {
            if tmp < n {
                break;
            }
            if ans > tmp {
                ans = tmp;
            }
            if j == 0 {
                break;
            }
            j -= 1;

            tmp = calc(i, j);
        }
    }

    println!("{}", ans);
}

#[inline]
fn calc(a: u64, b: u64) -> u64 {
    let a3 = a * a * a;
    let a2b = a * a * b;
    let ab2 = a * b * b;
    let b3 = b * b * b;
    a3 + a2b + ab2 + b3
}
