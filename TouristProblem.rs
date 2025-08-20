use std::io::{self, Read};

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

fn main() {
    // Fast input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();

    // sum(a)
    let sum_a: i128 = a.iter().map(|&x| x as i128).sum();

    // sort and compute pair_sum = sum_{i<j} (a_j - a_i)
    a.sort_unstable();
    let mut pref: i128 = 0; // prefix sum of previous elements
    let mut pair_sum: i128 = 0; // accumulates sum_{i<j}(a_j - a_i)

    for (i, &val) in a.iter().enumerate() {
        let v = val as i128;
        // Contribution of v as the "j" against all previous i's: i*v - pref
        pair_sum += v * (i as i128) - pref;
        pref += v;
    }

    // Expected distance = (sum_a + 2*pair_sum) / n
    let mut num = sum_a + 2 * pair_sum;
    let mut den = n as i128;

    let g = gcd(num, den);
    num /= g;
    den /= g;

    println!("{} {}", num, den);
}
