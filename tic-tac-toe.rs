// https://www.spoj.com/problems/TOE1/

use std::io::{self, Read};
fn win(board: &[[char; 3]; 3], p: char) -> bool {
    // rows
    for i in 0..3 {
        if board[i][0] == p && board[i][1] == p && board[i][2] == p {
            return true;
        }
    }
    // cols
    for j in 0..3 {
        if board[0][j] == p && board[1][j] == p && board[2][j] == p {
            return true;
        }
    }
    // diags
    if board[0][0] == p && board[1][1] == p && board[2][2] == p {
        return true;
    }
    if board[0][2] == p && board[1][1] == p && board[2][0] == p {
        return true;
    }
    false
}
fn is_valid(board: &[[char; 3]; 3]) -> bool {
    let mut cnt_x = 0;
    let mut cnt_o = 0;
    for i in 0..3 {
        for j in 0..3 {
            match board[i][i] {
                'X' => cnt_x += 1,
                'O' => cnt_o += 1,
                _ => {}
            }
        }
    }
    if !(cnt_x == cnt_o || cnt_x == cnt_o + 1) {
        return false;
    }
    let x_win = win(board, 'X');
    let o_win = win(board, 'O');

    if x_win && o_win {
        return false;
    }
    if x_win && cnt_x != cnt_o + 1 {
        return false;
    }
    if o_win && cnt_x != cnt_o {
        return false;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();
    let t: usize = it.next().unwrap().trim().parse().unwrap();
    let mut cases_processed = 0;
    while cases_processed < t {
        let mut line = it.next();
        while let Some(l) = line {
            // If this line is not empty after trimming, that means you’ve just reached the first meaningful line of the board.
            // break; then immediately exits the while loop, leaving line still set to this non‑empty line.
            if !l.trim().is_empty() {
                break;
            }
            line = it.next();
        }
        if line.is_none() {
            break;
        }

        let row1 = line.unwrap().trim().chars().collect::<Vec<_>>();
        let row2 = it.next().unwrap().trim().chars().collect::<Vec<_>>();
        let row3 = it.next().unwrap().trim().chars().collect::<Vec<_>>();
        let board = [
            [row1[0], row1[1], row1[2]],
            [row2[0], row2[1], row2[2]],
            [row3[0], row3[1], row3[2]],
        ];
        let mut cnt_x = 0;
        let mut cnt_o = 0;
        if is_valid(&board) {
            println!("yes");
        } else {
            println!("no");
        }

        cases_processed += 1;
    }
}
