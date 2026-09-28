use std::io::{self, Read}
fn dfs(
    grid: Vec<Vec<char>>,
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    vis: Vec<Vec<u32>>,
){
    vis[i][j] = 1;
    let drow = vec![-1,0,1,0];
    let dcol = vec![0,-1,0,1];

    for k in 0..4{
        let nrow = i as i32 + drow;
        let ncol = i as i32 + dcol;
        if nrow >= 0 && ncol >= 0 && nrow < n as i32 && ncol < m as i32 && vis[nrow as usize][ncol as usize] == 0 && grid[nrow as usize][ncol as usize] == '1' {
            dfs(grid, i , j, n , m, vis,);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    let mut grid = vec![vec!['0'; m];n];
    for i in 0..n{
        for j in 0..m{
            grid[i][j] = it.next().unwrap().chars().next().unwrap();
    
        }
    }
    let mut vis = vec![vec![0; m];n];
    let mut count =0 ;
    for i in 0..n{
        for j in 0..m{
            if !vis[i][j] && grid[i][j] == '1' {
                count+=1;
                dfs(&grid, i, j, n, m, &mut vis);
            }
        }
    }

    println!("{}", count);
}
