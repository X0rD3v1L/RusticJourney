pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;

        // Create adjacency list and in-degree array
        let mut adj = vec![Vec::new(); num_courses];
        let mut in_degree = vec![0; num_courses];

        // Build the graph
        for edge in prerequisites {
            let course = edge[0] as usize;
            let prereq = edge[1] as usize;
            adj[prereq].push(course);
            in_degree[course] += 1;
        }

        // Use a queue to store nodes with in-degree 0
        let mut queue = VecDeque::new();
        for i in 0..num_courses {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut count = 0;
        
        // Process nodes in topological order
        while let Some(course) = queue.pop_front() {
            count += 1;
            for &next_course in &adj[course] {
                in_degree[next_course] -= 1;
                if in_degree[next_course] == 0 {
                    queue.push_back(next_course);
                }
            }
        }

        // If all courses were processed, return true
        count == num_courses
    }
}

fn main() {
    let num_courses = 4;
    let prerequisites = vec![
        vec![1, 0], // Course 1 depends on Course 0
        vec![2, 1], // Course 2 depends on Course 1
        vec![3, 2], // Course 3 depends on Course 2
    ];
    println!("{}", Solution::can_finish(num_courses, prerequisites)); // Output: true

    let prerequisites_cycle = vec![
        vec![1, 0],
        vec![2, 1],
        vec![3, 2],
        vec![0, 3], // Cycle: 0 → 1 → 2 → 3 → 0
    ];
    println!("{}", Solution::can_finish(num_courses, prerequisites_cycle)); // Output: false
}
