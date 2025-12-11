// https://codeforces.com/contest/242/problem/C
use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin.read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let x0: i64 = it.next().unwrap().parse().unwrap();
    let y0: i64 = it.next().unwrap().parse().unwrap();
    let x1: i64 = it.next().unwrap().parse().unwrap();
    let y1: i64 = it.next().unwrap().parse().unwrap();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut allowed: HashSet<(i64, i64)> = HashSet::new();

    for _ in 0..n {
        let r: i64 = it.next().unwrap().parse().unwrap();
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        for c in a..b {
            allowed.insert((r, c))
        }
    }

    let start = (xo, y0);
    let target = (x1, y1);

    if !allowed.contains(&start) || !allowed.contains(&target) {
        println!("-1");
        return;
    }

    let mut q: VecDeque<((i64, i64), i32)> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    q.push_back((start, 0));
    visited.insert(start);
    let directions: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    while let Some(((x, y), dist)) = q.pop_front() {
        if (x, y) == target {
            println!("{}", dist);
            return;
        }
        for (dx, dy) in directions.iter() {
            let nx = x + dx;
            let ny = y + dy;
            let np = (nx, ny);
            if allowed.contains(&np) && !visited.contains(&np) {
                visited.insert(np);
                q.push_back((np, dist + 1))
            }
        }
    }
    println!("-1");
}
