// https://codeforces.com/contest/227/problem/B
use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();

    // Read n
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read array a
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Build a map: value -> position (1-based index)
    let mut pos = HashMap::new();
    for (i, &val) in a.iter().enumerate() {
        pos.insert(val, i + 1);
    }

    // Read m
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    // Read queries b
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Calculate Vasya’s and Petya’s total comparisons
    let mut vasya: u64 = 0;
    let mut petya: u64 = 0;

    for &x in &b {
        let p = *pos.get(&x).unwrap() as u64;
        vasya += p;
        petya += (n as u64 - p + 1);
    }

    println!("{} {}", vasya, petya);
}
