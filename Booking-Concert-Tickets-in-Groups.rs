use std::cmp::max;
use std::cmp::min;

struct BookMyShow {
    n: i32, // number of rows (example field)
    m: i32, // seats per row (example field)
    rem: Vec<i32>,
    tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    fn upd(&mut self, p: i32, v: i32, i: usize, tl: i32, tr: i32) {
        if tl == tr {
            self.tree[i] = v;
            return;
        }
        let m = (tl + tr) / 2;
        if p <= m {
            self.upd(p, v, 2 * i + 1, tl, m);
        } else {
            self.upd(p, v, 2 * i + 2, m + 1, tr);
            // self.tree[i] = max(self.tree[2 * i + 1], self.tree[2 * i + 2]);
        }
        self.tree[i] = max(self.tree[2 * i + 1], self.tree[2 * i + 2]);
    }

    fn updMini(&mut self, p: i32, v: i32) {
        self.upd(p, v, 0, 0, self.n - 1);
    }

    fn get(&mut self, k: i32, i: i32, tl: i32, tr: i32) -> i32 {
        if tl == tr {
            if self.tree[i as usize] >= k {
                return tl;
            }
            return -1;
        }
        let m = (tl + tr) / 2;
        if self.tree[(2 * i + 1) as usize] >= k {
            return self.get(k, 2 * i + 1, tl, m);
        } else if self.tree[(2 * i + 2) as usize] >= k {
            return self.get(k, 2 * i + 2, m + 1, tr);
        } else {
            return -1;
        }
    }

    //step 1
    fn BookMyShow(&mut self, nn: i32, mm: i32) {
        let n = nn;
        let m = nn;
        self.rem = vec![mm; nn as usize];
        self.tree = vec![mm; (4 * nn) as usize];
    }

    fn new(nn: i32, mm: i32) -> Self {
        let rem: Vec<i32> = vec![mm; nn as usize];
        let tree: Vec<i32> = vec![mm; (4 * nn) as usize];
        Self {
            n: nn,
            m: mm,
            rem,
            tree,
        }
    }

    fn gather(&mut self, k: i32, maxRow: i32) -> Vec<i32> {
        let pos: i32 = self.get(k, 0, 0, self.n - 1);
        if pos == -1 || pos > maxRow {
            Vec::new()
        } else {
            let x: Vec<i32> = vec![pos, self.m - self.rem[pos as usize]];
            self.rem[pos as usize] -= k;
            self.updMini(pos, self.rem[pos as usize]);
            return x;
        }
    }

    fn scatter(&mut self, mut k: i32, maxRow: i32) -> bool {
        let mut upds: Vec<(i32, i32)> = Vec::new();

        while k > 0 {
            let pos: i32 = self.get(1, 0, 0, self.n - 1);

            if pos == -1 || pos > maxRow {
                for &(row, taken) in &upds {
                    self.rem[row as usize] += taken;
                    self.updMini(row, self.rem[row as usize]);
                }
                return false;
            }
            let r = min(k, self.rem[pos as usize]);
            self.rem[pos as usize] -= r;
            k -= r;
            self.updMini(pos, self.rem[pos as usize]);
            upds.push((pos, r));
        }
        return true;
    }
}

fn main() {
    // vector of operations
    let ops = vec!["BookMyShow", "gather", "gather", "scatter", "scatter"];

    // vector of arguments
    let args = vec![vec![2, 5], vec![4, 0], vec![2, 0], vec![5, 1], vec![5, 1]];

    let mut out: Vec<String> = Vec::new();

    // process
    let mut obj = BookMyShow::new(args[0][0], args[0][1]);
    out.push("null".to_string()); // constructor output

    for i in 1..ops.len() {
        match ops[i] {
            "gather" => {
                let res = obj.gather(args[i][0], args[i][1]);
                if res.is_empty() {
                    out.push("[]".to_string());
                } else {
                    out.push(format!("[{}, {}]", res[0], res[1]));
                }
            }
            "scatter" => {
                let res = obj.scatter(args[i][0], args[i][1]);
                out.push(res.to_string());
            }
            _ => {}
        }
    }

    // print like expected output
    print!("[");
    for (i, s) in out.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", s);
    }
    println!("]");
}
