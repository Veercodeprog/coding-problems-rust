// https://www.spoj.com/problems/TOE2/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "end" {
            break;
        }

        if is_valid_board(&line) {
            println!("valid");
        } else {
            println!("invalid");
        }
    }
}

fn is_valid_board(board: &str) -> bool {
    let chars: Vec<char> = board.chars().collect();

    // Count X's and O's
    let count_x = chars.iter().filter(|&&c| c == 'X').count();
    let count_o = chars.iter().filter(|&&c| c == 'O').count();

    // X goes first, so count_x must be equal to count_o or count_o + 1
    if count_x != count_o && count_x != count_o + 1 {
        return false;
    }

    // Check if X or O wins
    let x_wins = has_won(&chars, 'X');
    let o_wins = has_won(&chars, 'O');

    // Both cannot win
    if x_wins && o_wins {
        return false;
    }

    // If X wins, X must have made the last move (count_x = count_o + 1)
    if x_wins && count_x != count_o + 1 {
        return false;
    }

    // If O wins, O must have made the last move (count_x = count_o)
    if o_wins && count_x != count_o {
        return false;
    }

    true
}

fn has_won(board: &[char], player: char) -> bool {
    // Check rows
    for i in 0..3 {
        if board[i * 3] == player && board[i * 3 + 1] == player && board[i * 3 + 2] == player {
            return true;
        }
    }

    // Check columns
    for i in 0..3 {
        if board[i] == player && board[i + 3] == player && board[i + 6] == player {
            return true;
        }
    }

    // Check diagonals
    if board[0] == player && board[4] == player && board[8] == player {
        return true;
    }
    if board[2] == player && board[4] == player && board[6] == player {
        return true;
    }

    false
}
