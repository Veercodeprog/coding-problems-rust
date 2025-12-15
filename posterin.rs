// https://www.spoj.com/problems/POSTERIN/
use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let N: i64 = input.trim().parse().unwrap();

    let mut stack: Vec<i64> = Vec::new();
    let mut posters: i64 = 0;
    for i in 0..N {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let mut it = input.split_whitespace();

        let _d: i64 = it.next().unwrap().parse().unwrap();

        let h: i64 = it.next().unwrap().parse().unwrap();
        while let Some(&top) = stack.last() {
            if top > h {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.last() != Some(&h) {
            stack.push(h);
            posters += 1;
        }
    }
    println!("{}", posters);
}
