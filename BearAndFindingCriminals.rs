// https://codeforces.com/contest/680/problem/B

use std::io;
fn sol(cities: Vec<i32>, m: usize, n: usize) -> i32 {
    let mut criminals_catched = cities[m];

    for d in 1..n {
        let left = if m >= d { Some(m - d) } else { None };
        let right = if m + d < n { Some(m + d) } else { None };

        match (left, right) {
            (Some(l), Some(r)) => {
                if cities[l] == 1 && cities[r] == 1 {
                    criminals_catched += 2;
                }
            }
            (Some(l), None) => criminals_catched += cities[l], // only left exists
            (None, Some(r)) => criminals_catched += cities[r], // only right exists
            (None, None) => break,                             // no more cities
        }
    }
    criminals_catched
}
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let nums: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let n = nums[0] as usize;
    let m = (nums[1] - 1) as usize; // 0-indexed
    s.clear();

    io::stdin().read_line(&mut s).unwrap();
    let cities: Vec<i32> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let ans = sol(cities, m, n);
    println!("{:}", ans);
}
