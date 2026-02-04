fn candy_crush(board: &mut Vec<Vec<i32>>) {
    let (rows, cols) = (board.len(), board[0].len());
    let mut should_continue = true;

    while should_continue {
        should_continue = false;

        for row in 0..rows {
            for col in 2..cols {
                if board[row][col] != 0 && board[row][col] == board[row][col - 1].abs() && board[row][col] == board[row][col - 2].abs() {
                    should_continue = true;
                    let candy_value = board[row][col].abs();
                    board[row][col] = -candy_value;
                    board[row][col - 1] = -candy_value;
                    board[row][col - 2] = -candy_value;
                }
            }
        }

        for col in 0..cols {
            for row in 2..rows {
                if board[row][col] != 0 && board[row][col] == board[row - 1][col].abs() && board[row][col] == board[row -2][col].abs() {
                    should_continue = true;
                    let candy_value = board[row][col];
                    board[row][col] = -candy_value;
                    board[row - 1][col] = -candy_value;
                    board[row - 2][col] = -candy_value;
                }
            }
        }

        if should_continue {
            for col in 0..cols {
                let mut write_position = rows - 1;
                for row in (0..=rows-1).rev() {
                    if board[row][col] > 0 {
                        board[write_position][col] = board[row][col];
                        write_position = write_position.saturating_sub(1);
                    }
                }

                while write_position  >= 0 as usize {
                    board[write_position][col] = 0;
                    write_position = write_position.saturating_sub(1);
                }
            }
        }
    }
}


fn main() {
    let mut board = vec![vec![1, 3, 5, 5, 2],
                                    vec![3, 4, 3, 3, 1],
                                    vec![3, 2, 4, 5, 2],
                                    vec![2, 4, 4, 5, 5],
                                    vec![1, 4, 4, 1, 1]];
    
    candy_crush(&mut board);
    print!("{:?}", board);
}