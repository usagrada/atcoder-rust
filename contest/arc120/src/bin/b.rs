#[allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

const MOD: u64 = 998244353;
#[derive(PartialEq, Clone, Copy)]
enum State {
    Red,
    Blue,
    None,
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [String; h],
    }

    let s: Vec<Vec<State>> = s
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'R' => State::Red,
                    'B' => State::Blue,
                    '.' => State::None,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut ans = 1;
    for index in 0..(h + w - 1) {
        let (x, y) = if index < h {
            (index, 0)
        } else {
            (h - 1, index - (h - 1))
        };
        let mut cnt = 0;
        let mut state = State::None;
        while x >= cnt && y + cnt < w {
            if s[x - cnt][y + cnt] == State::None {
            } else if state == State::None {
                state = s[x - cnt][y + cnt];
            } else if state != s[x - cnt][y + cnt] {
                ans = 0;
                state = s[x - cnt][y + cnt];
                break;
            }
            cnt += 1;
        }
        if state == State::None {
            ans *= 2;
            ans %= MOD;
        }
        // println!("{}, {}", index, ans);
    }
    println!("{}", ans % MOD);
}
