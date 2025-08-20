use std::collections::HashMap;
use std::io::{self, Read};

/// Lowest Common Ancestor in the implicit infinite binary tree (parent(i) = i/2).
pub fn lca(mut u: u64, mut v: u64) -> u64 {
    while u != v {
        if u > v {
            u >>= 1;
        } else {
            v >>= 1;
        }
    }
    u
}

/// Iterate all edges on the shortest path between u and v.
/// We represent an edge by its child endpoint (child -> parent(child)).
/// Calls `f(child)` once per edge along the path.
pub fn for_each_edge_on_path(mut u: u64, mut v: u64, mut f: impl FnMut(u64)) {
    while u != v {
        if u > v {
            f(u); // edge u -> (u/2)
            u >>= 1;
        } else {
            f(v); // edge v -> (v/2)
            v >>= 1;
        }
    }
}

/* ------- Optional helpers for the full problem (updates/queries) ------- */

type Weights = HashMap<u64, i128>; // weight of edge (child -> parent(child))

/// Add `w` to every edge on path(u, v).
pub fn add_on_path(weights: &mut Weights, u: u64, v: u64, w: i128) {
    for_each_edge_on_path(u, v, |child| {
        *weights.entry(child).or_default() += w;
    });
}

/// Sum weights over every edge on path(u, v).
pub fn sum_on_path(weights: &Weights, u: u64, v: u64) -> i128 {
    let mut s = 0i128;
    for_each_edge_on_path(u, v, |child| {
        if let Some(&w) = weights.get(&child) {
            s += w;
        }
    });
    s
}

fn main() {
    // Read entire stdin
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut it = buf.split_whitespace();

    //number of test cases
    let q: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut weights: Weights = HashMap::new();
    let mut out = String::new();

    for _ in 0..q {
        let t: u32 = it.next().unwrap().parse().unwrap();
        match t {
            // Type 1: 1 v u w  => increase all edges on path(v, u) by w
            2 => {
                let v: u64 = it.next().unwrap().parse().unwrap();
                let u: u64 = it.next().unwrap().parse().unwrap();
                let ans = sum_on_path(&weights, v, u);
                out.push_str(&format!("{}\n", ans));
            }

            1 => {
                let v: u64 = it.next().unwrap().parse().unwrap();
                let u: u64 = it.next().unwrap().parse().unwrap();
                let w: i128 = it.next().unwrap().parse::<i128>().unwrap();
                add_on_path(&mut weights, v, u, w);
            }
            // Type 2: 2 v u    => query sum of edges on path(v, u)
            _ => unreachable!(),
        }
    }

    print!("{out}");
}
