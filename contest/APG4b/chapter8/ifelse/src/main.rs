use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    if x < 10 {
        println!("x は 10 より小さい");
    }
    println!("終了");
}
