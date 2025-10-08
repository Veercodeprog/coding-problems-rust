use std::io;

fn sol(mut num: String) -> i32 {
    let mut cnt = 0;

    while num.len() > 1 {
        let sum: u32 = num.chars().map(|c| c.to_digit(10).unwrap()).sum();
        num = sum.to_string();
        cnt += 1;
    }

    cnt
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let num = s.trim().to_string();
    let ans = sol(num);
    println!("{}", ans);
}
