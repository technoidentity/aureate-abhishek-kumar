
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Q1: Generic Insertion Sort
fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

// Q2: Generic Selection Sort
fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        arr.swap(i, min_index);
    }
}

// Q3: Generic Binary Search
fn binary_search<T: Ord>(
    arr: &[T],
    target: &T,
) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && &arr[left] == target {
        Some(left)
    } else {
        None
    }
}

// Q4: Generic Merge Sort
fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let (left, right) = arr.split_at(mid);
    let mut result = Vec::with_capacity(arr.len());

    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    arr.clone_from_slice(&result);
}

// Q5: Generic Quick Sort
fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let last = arr.len() - 1;
    let mut pos = 0;

    for j in 0..last {
        if arr[j] <= arr[last] {
            arr.swap(pos, j);
            pos += 1;
        }
    }

    arr.swap(pos, last);
    pos
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = partition(arr);
    let (left, remainder) = arr.split_at_mut(pivot);

    quick_sort(left);
    quick_sort(&mut remainder[1..]);
}

// Q6: Generic Linked List
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: T) {
        self.head = Some(Box::new(ListNode {
            value,
            next: self.head.take(),
        }));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn len(&self) -> usize {
        let mut count = 0;
        let mut curr = self.head.as_deref();

        while let Some(node) = curr {
            count += 1;
            curr = node.next.as_deref();
        }

        count
    }
}

// Q7: Generic Binary Search Tree
struct BstNode<T> {
    value: T,
    left: Option<Box<BstNode<T>>>,
    right: Option<Box<BstNode<T>>>,
}

impl<T: Ord> BstNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left.as_mut() {
                Some(node) => node.insert(value),
                None => {
                    self.left = Some(Box::new(Self::new(value)));
                }
            }
        } else if value > self.value {
            match self.right.as_mut() {
                Some(node) => node.insert(value),
                None => {
                    self.right = Some(Box::new(Self::new(value)));
                }
            }
        }
    }

    fn search(&self, target: &T) -> Option<&T> {
        if target == &self.value {
            Some(&self.value)
        } else if target < &self.value {
            self.left.as_ref()?.search(target)
        } else {
            self.right.as_ref()?.search(target)
        }
    }

    fn inorder<'a>(&'a self, result: &mut Vec<&'a T>) {
        if let Some(left) = &self.left {
            left.inorder(result);
        }

        result.push(&self.value);

        if let Some(right) = &self.right {
            right.inorder(result);
        }
    }
}

// Q8: Generic Hash Table
struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0);

        let buckets = (0..capacity)
            .map(|_| Vec::new())
            .collect();

        Self { buckets }
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for (existing_key, existing_value) in bucket.iter_mut() {
            if existing_key == &key {
                *existing_value = value;
                return;
            }
        }

        bucket.push((key, value));
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);

        self.buckets[index]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
}

// Q9: Generic Maximum
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    let mut best = items.first()?;

    for item in &items[1..] {
        if item > best {
            best = item;
        }
    }

    Some(best)
}

// Q10: Lifetimes
fn longest<'a>(
    first: &'a str,
    second: &'a str,
) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

// Q11: Generic Stack with Traits
trait StackOperations<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
}

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
}

impl<T> StackOperations<T> for Stack<T> {
    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Q12: Generic Linear Search
fn linear_search<T: PartialEq>(
    arr: &[T],
    target: &T,
) -> Option<usize> {
    arr.iter().position(|x| x == target)
}

// ONE MAIN FUNCTION FOR ALL QUESTIONS
fn main() {
    // Q1
    let mut a = vec![8, 3, 5, 1, 4];
    insertion_sort(&mut a);
    println!("Q1 Insertion Sort: {a:?}");

    // Q2
    let mut b = vec![64, 25, 12, 22, 11];
    selection_sort(&mut b);
    println!("Q2 Selection Sort: {b:?}");

    // Q3
    println!(
        "Q3 Binary Search: {:?}",
        binary_search(&[2, 4, 6, 8, 10], &8)
    );

    // Q4
    let mut c = vec![9, 3, 7, 1, 6, 2];
    merge_sort(&mut c);
    println!("Q4 Merge Sort: {c:?}");

    // Q5
    let mut d = vec![10, 7, 8, 9, 1, 5];
    quick_sort(&mut d);
    println!("Q5 Quick Sort: {d:?}");

    // Q6
    let mut list = LinkedList::new();

    for x in [10, 20, 30] {
        list.push(x);
    }

    println!(
        "Q6 Linked List: len={}, pop={:?}",
        list.len(),
        list.pop()
    );

    // Q7
    let mut bst = BstNode::new(50);

    for x in [30, 70, 20, 40, 60, 80] {
        bst.insert(x);
    }

    let mut traversal = Vec::new();
    bst.inorder(&mut traversal);

    println!(
        "Q7 BST: search={:?}, inorder={traversal:?}",
        bst.search(&40)
    );

    // Q8
    let mut table = HashTable::new(10);
    table.insert("apple", 100);
    table.insert("banana", 200);
    table.insert("apple", 300);

    println!("Q8 Hash Table: {:?}", table.get(&"apple"));

    // Q9
    println!("Q9 Largest: {:?}", largest(&[3, 9, 2, 8]));

    // Q10
    println!(
        "Q10 Longest: {}",
        longest("programming", "rust")
    );

    // Q11
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);

    println!("Q11 Stack peek: {:?}", stack.peek());
    println!("Q11 Stack pop: {:?}", stack.pop());
    println!("Q11 Stack empty: {}", stack.is_empty());

    // Q12
    println!(
        "Q12 Linear Search: {:?}",
        linear_search(&[5, 7, 9], &7)
    );
}

// Unit Tests for Q12
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_existing_number() {
        assert_eq!(linear_search(&[2, 4, 6], &4), Some(1));
    }

    #[test]
    fn returns_none_for_missing_number() {
        assert_eq!(linear_search(&[2, 4, 6], &7), None);
    }

    #[test]
    fn handles_empty_slice() {
        assert_eq!(linear_search::<i32>(&[], &7), None);
    }

    #[test]
    fn finds_first_duplicate() {
        assert_eq!(
            linear_search(&[2, 4, 4, 6], &4),
            Some(1)
        );
    }

    #[test]
    fn works_with_strings() {
        assert_eq!(
            linear_search(&["cat", "dog"], &"dog"),
            Some(1)
        );
    }
}
