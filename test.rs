use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();

    let num: Vec<i32> = it
        .take(n as usize)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
}
