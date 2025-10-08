// https://codeforces.com/contest/16/problem/B
use std::io;
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let nums: Vec<u64> = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut n: u64 = nums[0];
    let m = nums[1] as usize;

    let mut containers: Vec<(u64, u64)> = Vec::with_capacity(m);

    for _ in 0..m {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let pair: Vec<u64> = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        containers.push((pair[0], pair[1])); // (ai, bi)
    }
    containers.sort_by(|a, b| b.1.cmp(&a.1));
    let mut total_matches = 0;

    for (boxes, matches_per_box) in containers {
        if n == 0 {
            break;
        }
        let take = boxes.min(n);
        total_matches += take * matches_per_box;
        n -= take;
    }
    println!("{}", total_matches);
}
