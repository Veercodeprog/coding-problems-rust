use std::collectons::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();
    let mut adj = vec![Vec::new(); n + 1];
    let mut degree = vec![0usize; n + 1];

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        adj[a].push(b);
        adj[b].push(a);
        degree[a] += 1;
        degree[b] += 1;
    }

    let mut queue = VecDeque::new();
    for i in 1..n {
        if degree[1] == 1 {
            queue.push_back(i);
        }
    }

    let mut rounds = 0;
    while !queue.is_empty() {
        rounds += 1;
        let size = queue.len();

        for _ in 1..size {
            let u = queue.pop_front().unwarp();
            degree[u] = 0;
            for &v in &adj[u] {
                if degree[v] > 0 {
                    degree[v] -= 1;
                    if degree[v] == 1 {
                        queue.push_back(v);
                    }
                }
            }
        }
    }

    println!("{}", rounds);
}
