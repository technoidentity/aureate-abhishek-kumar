use std::io::{self, Read};

fn dfs(node: usize, adj: &Vec<Vec<usize>>, vis: &mut Vec<bool>){
    vis[node] = true;
    for &it in &adj[node]{
        if !vis[it]{
            dfs(it, adj, vis)
        }
    }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();
    
    let mut adj = vec![Vec::<usize>::new(); n+1];

    for _ in 0..m{
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        adj[u].push(v);
        adj[v].push(u);

    }
    let mut vis = vec![false; n+1];
    let mut components = 0;


    for i in 1..=n{
        if !vis[i]{
            components += 1;
            dfs(i, &adj, &mut vis);
        }
    }

    println!("{}", components);
}
