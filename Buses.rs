use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAXM: usize = 100_000; // constraints only for reference
fn lower_bound(a: &[i64], x: i64) -> usize {
    // first index i with a[i] >= x
    let mut lo = 0usize;
    let mut hi = a.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
fn main() {
    // -------- read input fast --------
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // store buses
    let mut buses: Vec<(i64, i64)> = Vec::with_capacity(m);
    let mut points: Vec<i64> = vec![0]; // P starts with 0

    for _ in 0..m {
        let si: i64 = it.next().unwrap().parse().unwrap();
        let ti: i64 = it.next().unwrap().parse().unwrap();
        buses.push((si, ti));
        points.push(ti); // only end-stops go into P
    }

    // -------- coordinate compress P = {0} ∪ {t_i} --------
    points.sort_unstable();
    points.dedup();

    if points.binary_search(&n).is_err() {
        println!("0");
        return;
    }
    let mut idx_of: HashMap<i64, usize> = HashMap::with_capacity(points.len());
    for (i, &v) in points.iter().enumerate() {
        idx_of.insert(v, i);
    }
    let k = points.len();

    println!(" points: {:?}, buses: {:?}", points, buses);

    let mut idx_of: HashMap<i64, usize> = HashMap::with_capacity(points.len());
    for (i, &v) in points.iter().enumerate() {
        idx_of.insert(v, i);
    }
    let k = points.len();
    let mut grouped: Vec<Vec<usize>> = vec![Vec::new(); k];
    for &(s0, t0) in &buses {
        let r = *idx_of.get(&t0).unwrap();
        let l = lower_bound(&points, s0);
        grouped[r].push(l);
    }
    let mut f: Vec<i64> = vec![0; k]; // f[i] = ways to reach points[i]
    let mut pref: Vec<i64> = vec![0; k]; // pref[i] = sum_{j<=i} f[j]

    f[0] = 1;
    pref[0] = 1;

    for r in 1..k {
        let mut val = 0i64;
        let fr_1 = pref[r - 1];
        for &l in &grouped[r] {
            if l == 0 {
                val += fr_1;
            } else {
                let mut add = fr_1 + pref[l - 1];
                if add < 0 {
                    add += MOD;
                }
                val += add;
            }
            if val >= MOD {
                val -= MOD;
            }
        }
        f[r] = val % MOD;
        pref[r] = (pref[r - 1] + f[r]) % MOD;
    }
    let r_n = points.binary_search(&n).unwrap();
    println!("{}", f[r_n].rem_euclid(MOD));
}
