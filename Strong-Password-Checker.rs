use std::cmp::min;
struct Solution;
impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let n = chars.len();
        let mut g: Vec<i32> = Vec::new();

        let mut lower: bool = false;
        let mut upper: bool = false;
        let mut number: bool = false;

        for &c in &chars {
            if c >= 'a' && c <= 'z' {
                lower = true;
            }
            if c >= 'A' && c <= 'Z' {
                upper = true;
            }
            if c >= '0' && c <= '9' {
                number = true;
            }
        }
        let mut cnt = (lower as i32) + (upper as i32) + (number as i32);
        // println!("cnt: {:}", cnt);

        let mut l = 0;

        for i in 0..=n {
            if i == n || chars[i] != chars[l] {
                g.push((i - l) as i32);
                l = i;
            }
        }
        let mut tot = 0;
        let mut nn = n;
        if nn < 6 {
            while nn < 6 || cnt < 3 || g.iter().copied().max().unwrap_or(0) >= 3 {
                let mut found = false;
                for x in g.iter_mut() {
                    if *x >= 3 {
                        *x -= 2; //no three repeating characters in a row so if n< 6 a character can
                                 //occur either 4 or 5 times after if condition x>
                        cnt += 1;
                        nn += 1;
                        tot += 1;
                        found = true;
                    }
                }

                if !found {
                    cnt += 1;
                    nn += 1;
                    tot += 1;
                    // break;
                }
            }
        } else if n >= 6 && n <= 20 {
            while cnt < 3 || g.iter().copied().max().unwrap_or(0) >= 3 {
                let mut found = false;
                for x in g.iter_mut() {
                    if *x >= 3 {
                        *x -= 3;
                        tot += 1;
                        cnt += 1;
                        found = true;
                    }
                }
                if !found {
                    cnt += 1;
                    tot += 1;
                    // break;
                }
            }
        } else {
            let mut found = true;
            while nn > 20 && found {
                found = false;
                for x in g.iter_mut() {
                    if (*x >= 3) && (*x % 3 == 0) && (nn > 20) {
                        let ops = min(1 + 0, nn - 20);
                        *x -= ops as i32;
                        tot += ops as i32;
                        nn -= ops;

                        found = true;
                    }
                }
            }
            let mut found = true;
            while nn > 20 && found {
                found = false;
                for x in g.iter_mut() {
                    if (*x >= 3) && (*x % 3 == 1) && (nn > 20) {
                        let ops = min(1 + 1, nn - 20);

                        *x -= ops as i32;
                        tot += ops as i32;
                        nn -= ops;

                        found = true
                    }
                }
            }

            let mut found = true;
            while nn > 20 && found {
                found = false;
                for x in g.iter_mut() {
                    if (*x >= 3) && (*x % 3 == 2) && (nn > 20) {
                        let ops = min(1 + 2, nn - 20);

                        *x -= ops as i32;
                        tot += ops as i32;
                        nn -= ops;

                        found = true
                    }
                }
            }

            if (nn > 20) {
                tot += (nn as i32) - 20;
                nn = 20;
            }
            while cnt < 3 || g.iter().copied().max().unwrap_or(0) >= 3 {
                let mut found = false;
                for x in g.iter_mut() {
                    if *x >= 3 {
                        *x -= 3;
                        tot += 1;
                        cnt += 1;
                        // nn -= 1;
                        found = true;
                    }
                }
                if !found {
                    cnt += 1;
                    tot += 1;

                    // break;
                }
            }
        }
        // for x in &g {
        //     print!("{}", x);
        // }
        // println!();

        return tot;
    }
}

fn main() {
    let password = "1111111111".to_string();
    let result = Solution::strong_password_checker(password);
    println!("Result = {}", result);
}
