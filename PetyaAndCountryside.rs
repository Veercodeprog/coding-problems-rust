// https://codeforces.com/contest/66/problem/B

use std::io;

fn main() {
    let mut input = String::new();

    // Read n
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read heights
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let heights: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if n == 1 {
        println!("1");
        return;
    }

    let mut left = vec![1; n];
    let mut right = vec![1; n];

    for i in 1..n {
        if heights[i - 1] <= heights[i] {
            left[i] = left[i - 1] + 1;
        }
    }

    for i in (0..n - 1).rev() {
        if heights[i + 1] <= heights[i] {
            right[i] = right[i + 1] + 1;
        }
    }

    let max_watered = (0..n).map(|i| left[i] + right[i] - 1).max().unwrap();
    println!("{}", max_watered);
}
