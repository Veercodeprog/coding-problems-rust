// https://codeforces.com/contest/88/problem/B

use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let x: i64 = it.next().unwrap().parse().unwrap();
    let x2: i64 = x * x;

    let mut pos = vec![Vec::<(i64, i64)>::new(); 26];
    let mut shifts = Vec::<(i64, i64)>::new();

    for i in 0..n {
        let row = it.next().unwrap().as_bytes().to_vec(); //so suppose we type hbsd sjbdfjh it
                                                          //first split and take everything before white spcae that is hbsd using it.next() ,it
                                                          //converts it then to bytes and then in bytes vector
                                                          // println!("row vector: {:?}", row);

        for j in 0..m {
            let ch = row[j] as char; //from bytes vector gttting the value for every character
                                     // println!("ch : {}", ch);
            if ch == 'S' {
                shifts.push((i as i64, j as i64));
            } else if ch.is_ascii_lowercase() {
                let idx = (ch as u8 - b'a') as usize;
                pos[idx].push((i as i64, j as i64));
            }
        }
    }

    let q: usize = it.next().unwrap().parse().unwrap();
    let t = it.next().unwrap().as_bytes();
    let mut can_one_hand = vec![false; 26];
    let has_shift = !shifts.is_empty();
    if has_shift {
        for c in 0..26 {
            if pos[c].is_empty() {
                continue;
            }
            'outer: for &(ri, rj) in &pos[c] {
                for &(si, sj) in &shifts {
                    let di = ri - si;
                    let dj = rj - sj;
                    let d2 = di * di + dj * dj;
                    if d2 <= x2 {
                        can_one_hand[c] = true;
                        break 'outer;
                    }
                }
            }
        }
    }
    let mut need_other_hand = 0i64;

    for &chb in t {
        let ch = chb as char;
        if ch.is_ascii_lowercase() {
            let idx = (chb - b'a') as usize;
            if pos[idx].is_empty() {
                println!("-1");
                return;
            }
        } else if ch.is_ascii_uppercase() {
            let lower_idx = (chb.to_ascii_lowercase() - b'a') as usize;
            if lower_idx >= 26 || pos[lower_idx].is_empty() {
                println!("-1");
                return;
            }
            if !has_shift {
                println!("-1");
                return;
            }
            if !can_one_hand[lower_idx] {
                need_other_hand += 1;
            }
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", need_other_hand);
}
