#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }
    // let mut t = t;
    let mut ans: u64 = 0;

    let tmax = 41;
    let mut flags = vec![0; tmax];
    let mut cnt = 0;

    for i in 0..n {
        let value: u64 = 1 << t[i];
        // bit が立ってない
        if ans & value == 0 {
            // 下位bit を消す
            ans -= ans & (value - 1);
            ans += value;
            for i in 0..t[i] {
                flags[i] = 0;
            }
            flags[t[i]] = 1;
        } else {
            let mut bitpos = t[i] + 1;
            loop {
                if bitpos == tmax {
                    cnt += 2;
                    ans = cnt << (tmax - 1);
                    ans += value;
                    for j in 0..tmax {
                        flags[j] = 0;
                    }
                    flags[t[i]] = 1;
                    break;
                }

                if flags[bitpos] == 0 {
                    ans -= ans & ((1 << bitpos) - 1);
                    ans += 1 << bitpos;
                    ans += value;
                    for j in 0..bitpos {
                        flags[j] = 0;
                    }
                    flags[t[i]] = 1;
                    flags[bitpos] = 1;
                    break;
                }
                bitpos += 1;
            }
        }
        assert!(ans & (value - 1) == 0);
    }

    println!("{}", ans);
}
