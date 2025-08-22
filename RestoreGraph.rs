use std::io::{self, Read};

fn main() {
    // -------- Fast input --------
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: i32 = it.next().unwrap().parse().unwrap();

    let mut d: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: usize = it.next().unwrap().parse().unwrap();
        d.push(x);
    }

    // -------- Build layers L[h] --------
    let &max_d = match d.iter().max() {
        Some(m) => m,
        None => {
            // No vertices? Not possible due to constraints, but handle defensively.
            println!("-1");
            return;
        }
    };

    let mut layers: Vec<Vec<usize>> = vec![Vec::new(); max_d + 1];
    for (i, &di) in d.iter().enumerate() {
        layers[di].push(i + 1); // store 1-based vertex index
    }

    // -------- Feasibility checks --------
    // 1) Exactly one root at distance 0
    if layers[0].len() != 1 {
        println!("-1");
        return;
    }
    // 2) No gaps: if L[h] nonempty => L[h-1] nonempty
    for h in 1..=max_d {
        if !layers[h].is_empty() && layers[h - 1].is_empty() {
            println!("-1");
            return;
        }
    }

    // -------- Construction with capacities --------
    // cap[u] = remaining "child slots" u can still host
    // Root starts with k, any newly attached non-root starts with k-1.
    let mut cap = vec![0i32; n + 1];
    let root = layers[0][0];
    cap[root] = k;

    let mut edges: Vec<(usize, usize)> = Vec::new();
    edges.reserve(n.saturating_sub(1)); // at most n-1 edges for a tree-like build

    for h in 1..=max_d {
        // Prepare queue of parents from previous layer that still have capacity
        let mut parents: Vec<usize> = Vec::new();
        for &p in &layers[h - 1] {
            if cap[p] > 0 {
                parents.push(p);
            }
        }
        if parents.is_empty() && !layers[h].is_empty() {
            // Nonempty current layer but no eligible parents
            println!("-1");
            return;
        }

        let mut pi = 0; // index into parents
        for &v in &layers[h] {
            // Advance to a parent with remaining capacity
            while pi < parents.len() && cap[parents[pi]] == 0 {
                pi += 1;
            }
            if pi == parents.len() {
                // Ran out of parent capacity; impossible
                println!("-1");
                return;
            }
            let p = parents[pi];
            edges.push((p, v));
            cap[p] -= 1;

            // New child gets capacity k-1
            cap[v] = k - 1;
        }
    }

    // -------- Output --------
    println!("{}", edges.len());
    for (a, b) in edges {
        println!("{} {}", a, b);
    }
}

