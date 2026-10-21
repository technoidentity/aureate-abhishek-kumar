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

fn merge_sort(arr2 : &mut Vec<i32>) -> Vec<i32> {
        
}



fn main() {
    let mut values = vec![5, 2, 4, 1, 3];
    let sorted = selection_sort(&mut values);

    println!("sorted: {sorted:?}");
}



