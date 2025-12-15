// https://acm.timus.ru/problem.aspx?space=1&num=1638
//
use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let v = nums[0]; // thickness without covers
    let c = nums[1]; // one cover thickness
    let a = nums[2];
    let b = nums[3];

    let d = (a - b).abs();
    let ans = if d == 0 {
        v
    } else {
        2 * c + (d - 1) * (v + 2 * c)
    };
    println!("{ans}");
}
