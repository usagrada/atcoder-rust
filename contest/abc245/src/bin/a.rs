use proconio::input;
fn main() {
    input! {
      a: usize,
      b: usize,
      c: usize,
      d: usize,
    }

    let takahashi = "Takahashi";
    let aoki = "Aoki";
    if a < c {
        println!("{}", takahashi);
    } else if a == c {
        if b <= d {
            println!("{}", takahashi);
        } else {
            println!("{}", aoki);
        }
    } else {
        println!("{}", aoki);
    }
}
