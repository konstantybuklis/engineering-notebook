use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut set_row: HashSet<char> = HashSet::new();
            let mut set_column: HashSet<char> = HashSet::new();

            for j in 0..9 {
                if board[i][j] != '.' {
                    if set_row.contains(&board[i][j]) {
                        return false;
                    }

                    set_row.insert(board[i][j]);
                }

                if board[j][i] != '.' {
                    if set_column.contains(&board[j][i]) {
                        return false;
                    }

                    set_column.insert(board[j][i]);
                }
            }
        }

        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let mut set_box: HashSet<char> = HashSet::new();

                for k in i..i + 3 {
                    for l in j..j + 3 {
                        if board[k][l] != '.' {
                            if set_box.contains(&board[k][l]) {
                                return false;
                            }

                            set_box.insert(board[k][l]);
                        }
                    }
                }
            }
        }

        true
    }
}

fn main() {
    let board: Vec<Vec<char>> = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let answer = Solution::is_valid_sudoku(board);
    println!("{answer}");
}
