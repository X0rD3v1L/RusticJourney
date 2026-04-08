fn dfs(city: usize, is_connected: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
    visited[city] = true;

    for neighbour in 0..is_connected.len() {
        if is_connected[city][neighbour] == 1 && !visited[neighbour] {
            dfs(neighbour, is_connected, visited);
        }
    }
}

fn find_connected_components(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut visited = vec![false; n];
    let mut provinces = 0;

    for i in 0..n {
        if !visited[i] {
            provinces += 1;
            dfs(i, &is_connected, &mut visited);
        }
    }
    provinces
}

fn main(){
    let is_connected = vec![
        vec![1, 1, 0],
        vec![1, 1, 0],
        vec![0, 0, 1]
    ];
    println!("{}", find_connected_components(is_connected));
}