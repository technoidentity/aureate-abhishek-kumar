use std::io::{self, Read};

fn check(
    node: i32,
    mn: i64,
    mx: i64,
    tree: &Vec<(i64, i32, i32)>
)->bool {
    if node == -1{
        return true
    }
    let (val, left, right) = tree[node as usize];

    if val <= mn || val >= mx {
        return false;

    }

    check(left, mn , val, tree) && check(right, val, mx, tree)
}
fn solve(tree: &Vec<(i64, i32, i32)>) -> bool {
    check(0, i64::MIN, i64::MAX, tree)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut tree = Vec::new();

    for _ in 0..n {
        let val: i64 = iter.next().unwrap().parse().unwrap();
        let left: i32 = iter.next().unwrap().parse().unwrap();
        let right: i32 = iter.next().unwrap().parse().unwrap();

        tree.push((val, left, right));
    }

    if n == 0 || solve(&tree) {
        println!("YES");
    } else {
        println!("NO");
    }
}