use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut a_index = 0;

    for i in 1..n + 1 {
        println!("{}", a[a_index] - i);

        if a[a_index] == i {
            a_index += 1;
        }
    }
}
