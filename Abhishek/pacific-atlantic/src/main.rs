use std::io::{self, Read};
fn dfs_atlantic(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    grid: Vec<Vec<i:32>>,
    vis: Vec<Vec<i:32>>,
){
    vis[i][j] = 1;
    let drow = vec![-1,0,1,0];
    let dcol = vec![0,-1,0,1];
    for k in 0..4{
        let nrow = i as i32 + drow[k];
        let ncol = j as i32 + drow[k];
        if(
            nrow >= 0
            && nocl >=0
            && nrow < n
            && ncol < m 
            && vis[nrow as usize][]
        )
    }
}
fn main() {
    println!("Hello, world!");
}
