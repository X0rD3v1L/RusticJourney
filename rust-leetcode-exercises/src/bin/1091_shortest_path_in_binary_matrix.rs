use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn validate(matrix: &[Vec<i32>], visited: &[Vec<bool>], row: isize, col: isize) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        if row < 0 || col < 0 || row >= m as isize || col >= n as isize {
            return false;
        }
        let (r, c) = (row as usize, col as usize);
        matrix[r][c] == 0 && !visited[r][c]
    }

    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [(isize, isize); 8] = [
            (-1,  1), ( 0,  1), ( 1,  1),
            (-1,  0),           ( 1,  0),
            (-1, -1), ( 0, -1), ( 1, -1),
        ];

        let m = grid.len();

        if m == 0 || grid[0][0] != 0 || grid[m - 1][m - 1] != 0 {
            return -1;
        }

        let mut visited = vec![vec![false; m]; m];
        let mut q = VecDeque::new();
        visited[0][0] = true;
        q.push_back((0, 0, 1)); // row, col, distance

        while let Some((i, j, dist)) = q.pop_front() {
            if i == m - 1 && j == m - 1 {
                return dist;
            }

            for (dr, dc) in DIRECTIONS.iter() {
                let row = i as isize + dr;
                let col = j as isize + dc;

                if Self::validate(&grid, &visited, row, col) {
                    let (r, c) = (row as usize, col as usize);
                    visited[r][c] = true;
                    q.push_back((r, c, dist + 1));
                }
            }
        }

        -1
    }
}

fn main() {
    let grid = vec![
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
        vec![1, 1, 1, 1, 1, 0],
    ];

    let result = Solution::shortest_path_binary_matrix(grid);
    println!("Shortest path length: {}", result);
}