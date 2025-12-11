use std::io;

fn main() {
    let mut input = String::new();

    // read first line (n)
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // clear the buffer before reading the next line
    input.clear();

    // read second line (array)
    io::stdin().read_line(&mut input).unwrap();
    let mut a: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a.sort_unstable();

    for i in 0..n.saturating_sub(2) {
        if a[i].saturating_add(a[i + 1]) > a[i + 2] {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
