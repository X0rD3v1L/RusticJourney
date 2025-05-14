use std::collections::VecDeque;

fn bfs(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, rows: i32, cols: i32) {
    grid[row as usize][col as usize] = 2;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = VecDeque::new();
    queue.push_front((row, col));

    while let Some((x, y)) = queue.pop_front() {
        for direction in directions {
            let new_x = x + direction.0;
            let new_y = y + direction.1;

            while new_x >= 0 && new_x < rows && new_y >=0 && new_y < cols && grid[new_x as usize][new_y as usize] == 1 {
                grid[new_x as usize][new_y as usize] = 2;
                queue.push_back((new_x, new_y));
            }
        }
    }
}

fn main() {
    let mut grid = vec![
        vec![1, 1, 0, 1, 1],
        vec![1, 1, 0, 0, 0]
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 1 {
                bfs(&mut grid, row as i32, col as i32, rows as i32, cols as i32);
                count += 1;
            }
        }
    }

    println!("{}", count);
}