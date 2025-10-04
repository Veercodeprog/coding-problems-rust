use std::io;

fn main() {
    // read n
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // arr keeps the input order
    let mut arr: Vec<i32> = Vec::with_capacity(n);
    // sorted_arr keeps sorted order
    let mut sorted_arr: Vec<i32> = Vec::with_capacity(n);

    while arr.len() < n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        for num_str in input.split_whitespace() {
            let num: i32 = num_str.parse().unwrap();
            arr.push(num);

            // keep sorted_arr sorted as we insert
            let pos = sorted_arr.binary_search(&num).unwrap_or_else(|e| e);
            sorted_arr.insert(pos, num);
        }
    }

    // find repeating numbers
    let mut duplicates = Vec::new();
    for i in 1..sorted_arr.len() {
        if sorted_arr[i] == sorted_arr[i - 1] {
            if duplicates.last() != Some(&sorted_arr[i]) {
                duplicates.push(sorted_arr[i]);
            }
        }
    }

    // print output
    if duplicates.is_empty() {
        println!("No repeating numbers");
    } else {
        for (i, num) in duplicates.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", num);
        }
        println!();
    }
}
