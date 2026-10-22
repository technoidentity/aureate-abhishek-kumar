use std::io::{self, Read};
fn selection_sort(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::with_capacity(arr.len());

    // Keep selecting and removing the smallest remaining value.

    while !arr.is_empty() {
        let mini = arr
                    .iter()
                    .enumerate()
                    .min_by_key(|(_,value)|*value)
                    .map(|(index, _)|index)
                    .expect("arr is known to be non-empty");
        stack.push(arr.remove(mini));
        // println!("{:?}", stack);
                    
    }

    stack
}

fn bubble_sort(arr2 : &mut Vec<i32>){
    let n = arr2.len();
    for i in 0..n{
        for j in 0..n-i-1 {
            if arr2[j]>arr2[j+1] {
                arr2.swap(j,j+1);
            }
        }
    }
}

fn two_sum(arr: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    if arr.len() < 2{
        return None;
    }
    let n = arr.len();
    let mut l = 0;
    let mut r = n-1;
    while l < r{
        let sum = arr[l] as i64 + arr[r] as i64;
        if sum == target as i64 {
            return Some((arr[l], arr[r]));
        }else if sum < target as i64 {
            l+=1;
        }else{
            r-=1;
        }

    }
    None
}

fn subset_sum(
    arr3: &Vec<i32>,
    target: i32,
    i: usize,
    subset: &mut Vec<i32>,
    ans: &mut Vec<Vec<i32>>,
    dp: &mut Vec<Vec<i32>>,
) -> i32{
    if i == arr3.len(){
        if target == 0 {
            return 1;

        }
        return 0;
    }
    if target<0{
        return 0;
    }
    if dp[i][target as usize] != -1{
        return dp[i][target as usize];
    }
    // subset.push(arr3[i]);

    // subset_sum(arr3, target-arr3[i], i+1, subset, ans);

    // subset.pop();

    // subset_sum(arr3, target, i+1, subset, ans);

    let pick = subset_sum(arr3, target - arr3[i], i+1, subset, ans, dp);
    let not_pick = subset_sum(arr3, target , i+1, subset, ans, dp);
    
    dp[i][target as usize] = pick + not_pick;

    dp[i][target as usize]

}

fn solve(nums:&Vec<i32>,
        ans : &mut Vec<i32>){
    let n = nums.len();
    
    for i in 0..2*n{
        ans[i] = nums[i%n];
    }
    
}
fn shuffle(
    nums: &Vec<i32>,
    ans: &mut Vec<i32>
){
    let n = nums.len();
    for i in 0..n/2{
        ans.push(nums[i]);
        ans.push(nums[n/2+i]);
    }
}


fn main() {
    let mut values = vec![5, 2, 4, 1, 3];
    let sorted = selection_sort(&mut values);

    println!("sorted: {sorted:?}");

    let mut arr2 = vec![5,4,2,3,1];
    bubble_sort(& mut arr2);
    println!("{:?}", arr2);
    println!("{:?}", two_sum(arr2, 5));
    let mut subset = Vec::new();
    let mut ans = Vec::new(); 
    let arr3 = vec![5,4,2,3,1];
    let mut target = 5;
    let mut dp = vec![vec![-1; target as usize + 1]; arr3.len()];
    let result = subset_sum(&arr3, target, 0, &mut subset, &mut ans, &mut dp);
    println!("Number of subsets: {}", result);
    let nums = vec![1,2,1];
    let n = nums.len();
    let mut ans = vec![0; 2 * n];
    solve(&nums, &mut ans);
    println!("{:?}", ans);
    let mut ans2: Vec<i32> = Vec::new();
    let nums2 = vec![5,4,3,2,1,7];
    shuffle(&nums2, &mut ans2);
    println!("{:?}", ans2);

}



