// https://codeforces.com/contest/78/problem/B

use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let base = "ROYGBIV";
    let extra = "GBIV";
    let mut result = String::new()
        result.push_str(base);
    if n> 7 {
        let remaining = n-7;
        for i in o..7{
            result.push(extra.chars().nth(i%4).unwrap());
        }
    }
    println!("{}", result);

}
