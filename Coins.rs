// https://codeforces.com/contest/47/problem/B
//
//
use std::collections::HashMap;
use std::io;

fn main() {
    let mut comparisons = Vec::new();
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let chars: Vec<char> = trimmed.chars().collect();
        if chars.len() == 3 {
            comparisons.push(chars);
        }
    }
    let mut coins: HashMap<char, i32> = HashMap::new();
    coins.insert('A', 0);
    coins.insert('B', 0);
    coins.insert('C', 0);
    for i in 0..3 {
        if comparisons[i][1] == '>' {
            *coins.get_mut(&comparisons[i][0]).unwrap() += 1;
        } else {
            *coins.get_mut(&comparisons[i][2]).unwrap() += 1;
        }
    }

    let mut coin_vec: Vec<(&char, &i32)> = coins.iter().collect();
    coin_vec.sort_by(|a, b| a.1.cmp(b.1));

    if coin_vec[0].1 == coin_vec[1].1 || coin_vec[1].1 == coin_vec[2].1 {
        println!("Impossible");
    } else {
        for (coin, _) in coin_vec {
            print!("{}", coin);
        }
        println!();
    }
}
