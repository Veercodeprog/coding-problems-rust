use std::io::{self, Read};

const MAXK: usize = 30;

fn main() {
    // ---- input ----
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // level[k] = number of free blocks of size 2^k
    let mut level = vec![0u64; MAXK + 1];

    // decompose clusters into powers of two
    for _ in 0..n {
        let mut x: u32 = it.next().unwrap().parse().unwrap();
        while x != 0 {
            let k = x.trailing_zeros() as usize;
            level[k] += 1;
            x &= x - 1;
        }
    }

    // need[k] = number of arrays of size 2^k
    let mut need = vec![0u64; MAXK + 1];
    for _ in 0..m {
        let b: usize = it.next().unwrap().parse().unwrap();
        if b <= MAXK {
            need[b] += 1;
        }
    }

    let mut ans: u64 = 0;

    // Serve small -> large
    for k in 0..=MAXK {
        // 1) Use native at level k
        let take = need[k].min(level[k]);
        need[k] -= take;
        level[k] -= take;
        ans += take;

        // 2) While we still need 2^k blocks:
        while need[k] > 0 {
            // consume any stock created by previous pulls
            if level[k] > 0 {
                let t = need[k].min(level[k]);
                need[k] -= t;
                level[k] -= t;
                ans += t;
                continue;
            }

            // find nearest higher level with stock
            let mut j_opt = None;
            for j in (k + 1)..=MAXK {
                if level[j] > 0 {
                    j_opt = Some(j);
                    break;
                }
            }
            let Some(j) = j_opt else {
                // no more memory to satisfy size 2^k
                break;
            };

            // Pull ONE block from level j and split down to k:
            // - level[j] -= 1
            // - leftover siblings at intermediate levels
            // - splitting yields TWO blocks of size 2^k
            level[j] -= 1;
            for t in (k + 1)..=j - 1 {
                level[t] += 1;
            }
            level[k] += 2;

            // Immediately consume one produced 2^k
            level[k] -= 1;
            need[k] -= 1;
            ans += 1;
        }
    }

    println!("{}", ans);
}
