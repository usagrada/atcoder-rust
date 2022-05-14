use proconio::input;
#[derive(PartialEq, Eq, Clone, Debug)]
enum Flag {
    Ab,
    A,
    B,
    None,
}
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let mut check_flag = vec![Flag::None; n];
    let mut ans_flag = true;
    check_flag[0] = Flag::Ab;
    for i in 0..(n - 1) {
        if check_flag[i] == Flag::Ab {
            if (a[i + 1] - a[i]).abs() <= k || (a[i + 1] - b[i]).abs() <= k {
                check_flag[i + 1] = Flag::A;
            }
            if (b[i + 1] - a[i]).abs() <= k || (b[i + 1] - b[i]).abs() <= k {
                if check_flag[i + 1] == Flag::A {
                    check_flag[i + 1] = Flag::Ab;
                    continue;
                }
                check_flag[i + 1] = Flag::B;
            }
        }
        if check_flag[i] == Flag::A {
            if (a[i + 1] - a[i]).abs() <= k {
                check_flag[i + 1] = Flag::A;
            }
            if (b[i + 1] - a[i]).abs() <= k {
                if check_flag[i + 1] == Flag::A {
                    check_flag[i + 1] = Flag::Ab;
                    continue;
                }
                check_flag[i + 1] = Flag::B;
            }
        }

        if check_flag[i] == Flag::B {
            if (a[i + 1] - b[i]).abs() <= k {
                check_flag[i + 1] = Flag::A;
            }
            if (b[i + 1] - b[i]).abs() <= k {
                if check_flag[i + 1] == Flag::A {
                    check_flag[i + 1] = Flag::Ab;
                    continue;
                }
                check_flag[i + 1] = Flag::B;
            }
        }
        if check_flag[i + 1] == Flag::None {
            ans_flag = false;
        }
    }

    let y = "Yes";
    let n = "No";
    if ans_flag {
        println!("{}", y);
    } else {
        println!("{}", n);
    }
}
