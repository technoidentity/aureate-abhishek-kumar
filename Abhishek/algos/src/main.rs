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

fn 

fn main() {
    let mut values = vec![5, 2, 4, 1, 3];
    let sorted = selection_sort(&mut values);

    println!("sorted: {sorted:?}");

    let mut arr2 = vec![5,4,2,3,1];
    bubble_sort(& mut arr2);
    println!("{:?}", arr2);
}



