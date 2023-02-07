#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let ls = s.len();
    let lt = t.len();
    let mut flags_left = vec![false; lt];
    let mut flags_right = vec![false; lt];
    for i in 0..lt {
        // left side
        if &s[i..i + 1] == "?" || &t[i..i + 1] == "?" {
            flags_left[i] = true;
        } else if s[i..i + 1] == t[i..i + 1] {
            flags_left[i] = true;
        }

        // right side
        if &s[ls - i - 1..ls - i] == "?" || &t[lt - i - 1..lt - i] == "?" {
            flags_right[lt - i - 1] = true;
        } else if s[ls - i - 1..ls - i] == t[lt - i - 1..lt - i] {
            flags_right[lt - i - 1] = true;
        }
    }

    let check_left = {
        let mut vec = vec![false; lt + 1];
        vec[0] = true;
        let mut now = true;
        for i in 0..lt {
            vec[i + 1] = now && flags_left[i];
            now = vec[i + 1];
        }
        vec
    };
    let check_right = {
        let mut vec = vec![false; lt + 1];
        vec[lt] = true;
        let mut now = true;
        for i in 0..lt {
            vec[lt - i - 1] = now && flags_right[lt - i - 1];
            now = vec[lt - i - 1];
        }
        vec
    };
    // eprintln!("l: {:?}", flags_left);
    // eprintln!("r: {:?}", flags_right);
    // eprintln!("l: {:?}", check_left);
    // eprintln!("r: {:?}", check_right);

    for x in 0..=lt {
        if check_left[x] && check_right[x] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
