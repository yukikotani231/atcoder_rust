use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        s: String,
        t: String,
    }

    let is_prefix = t.starts_with(&s);
    let is_suffix = t.ends_with(&s);

    let ans = match (is_prefix, is_suffix) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };

    println!("{}", ans);
}
