struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    if !Self::is_valid(&board, r, c, board[r][c]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, r: usize, c: usize, ch: char) -> bool {
        for j in 0..9 {
            if j != c && board[r][j] == ch {
                return false;
            }
        }

        for i in 0..9 {
            if i != r && board[i][c] == ch {
                return false;
            }
        }

        let row_num = 3 * (r / 3);
        let col_num = 3 * (c / 3);

        for i in row_num..row_num + 3 {
            for j in col_num..col_num + 3 {
                if (i != r || j != c) && board[i][j] == ch {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];

    println!("{}", Solution::is_valid_sudoku(board));
}
