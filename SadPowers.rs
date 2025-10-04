use std::io::{self, Read};

const LIMIT: u128 = 1_000_000_000_000_000_000; // 1e18

fn main() {
    // ---- read all input ----
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let q: usize = it.next().unwrap().parse().unwrap();

    // ---- precompute: all perfect powers (p >= 3) that are NOT squares ----
    let others = precompute_non_square_powers();

    // ---- answer queries ----
    let mut out = String::new();
    for _ in 0..q {
        let l: u128 = it.next().unwrap().parse::<u128>().unwrap();
        let r: u128 = it.next().unwrap().parse::<u128>().unwrap();

        // count perfect squares in [l, r]
        let squares = count_squares(l, r);

        // count non-square higher powers in [l, r] using binary search
        let lo = lower_bound(&others, l);
        let hi = upper_bound(&others, r);
        let non_squares = (hi - lo) as u128;

        let ans = squares + non_squares;
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}

// ---------- precompute a^p (p odd >=3), skipping squares, dedup ----------
fn precompute_non_square_powers() -> Vec<u128> {
    let mut v: Vec<u128> = Vec::new();

    // p up to 59 since 2^60 > 1e18
    for p in (3u32..=59).step_by(2) {
        let mut base = 2u128;
        loop {
            match pow_limit(base, p) {
                Some(val) => {
                    if !is_square(val) {
                        v.push(val);
                    }
                    base += 1;
                }
                None => break, // base^p > LIMIT -> higher bases will also exceed
            }
        }
    }

    v.sort_unstable();
    v.dedup();
    v
}

// Compute base^p but stop and return None if it exceeds LIMIT
fn pow_limit(base: u128, p: u32) -> Option<u128> {
    let mut res = 1u128;
    for _ in 0..p {
        // Avoid overflow and respect the LIMIT guard
        res = res.saturating_mul(base);
        if res > LIMIT {
            return None;
        }
    }
    Some(res)
}

// --------- integer sqrt helpers (robust for up to 1e18) ----------
fn floor_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    // Start with f64 sqrt estimate, then correct
    let mut r = (n as f64).sqrt() as u128;
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    while r * r > n {
        r -= 1;
    }
    r
}

fn is_square(n: u128) -> bool {
    let r = floor_sqrt(n);
    r * r == n
}

fn count_squares(l: u128, r: u128) -> u128 {
    let a = floor_sqrt(r);
    let b = if l == 0 { 0 } else { floor_sqrt(l - 1) };
    a - b
}

// --------- binary search helpers for inclusive range [L, R] ----------
fn lower_bound(a: &[u128], x: u128) -> usize {
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

fn upper_bound(a: &[u128], x: u128) -> usize {
    let mut lo = 0usize;
    let mut hi = a.len();
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] <= x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}
