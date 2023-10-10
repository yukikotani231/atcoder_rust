use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }

    let index = s.find("ABC");

    match index {
        Some(i) => println!("{}", i + 1),
        None => println!("-1"),
    }
}
