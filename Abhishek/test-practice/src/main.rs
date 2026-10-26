// fn get_long_name(name: String) -> String {
//     name
// }

// fn main() {
//     let name = String::from("Rust Programming");

//     let result = get_long_name(name.clone());

//     println!("{}", name);
//     println!("{}", result);
// }



// fn find_number(numbers: &[i32], target: i32) -> Option<i32> {
//     // TODO
//     let n = numbers.len();


//     for i in 0..n{
//         if target == numbers[i] {
//             return Some(numbers[i]);
//         }
        
//     }
//     None
// }
// fn main() {
//     let numbers = vec![10, 20, 30, 40 ,50];
//     let result = find_number(&numbers, 20);
//     println!("{:?}", result);
// }

// use std::io;


// fn main(){
//     let mut scores : Vec<i32> = Vec::new();
//     let n = 5;
//     let mut sum = 0;
    
//     for i in 0..5{
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read input");

//         let num: i32 = input.trim().parse().unwrap();
//         scores.push(num);
//         print!("{} ", num);
//         sum += num;

//     }
//     let maxi = scores.iter().max();
//     println!("\n{:?}", maxi);

// }


// use std::collections::{HashMap, HashSet, LinkedList};
// enum PlayerRole {
//     Batsman,
//     Bowler,
//     AllRounder,
//     WicketKeeper,
// }

// struct Player {
//     name: String,
//     role: PlayerRole,
//     runs: i32,
//     wickets: i32,
// }

// impl Player {
//     fn new(name: &str, role: PlayerRole) -> Self {
//         // TODO
//         Self{
//             name: String::from(name),
//             role,
//             runs:0,
//             wickets:0,
//         }

//     }

//     fn add_runs(&mut self, runs: i32) {
//         // TODO
//         self.runs += runs;
//     }

//     fn add_wicket(&mut self) {
//         // TODO
//         self.wickets += 1;
//     }

//     fn print(&self) {
//         // TODO
//         println!(
//             "{} : Runs , {} : Wickets", self.runs, self.wickets
//         );
//     }
// }

// fn main() {
//     let mut player = Player::new("Virat", PlayerRole::Batsman);

//     player.add_runs(85);
//     player.add_wicket();

//     player.print();
// }



struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}


impl LinkedList {
    fn new() -> Self {
        // TODO
        Self{
            head: None,
        }
    }

    fn push(&mut self, value: i32) {
        // TODO
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn print(&self) {
        // TODO
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("{}", node.value);

            current = node.next.as_ref();
        }
    }
}

let mut list = LinkedList::new();

list.push(10);
list.push(20);
list.push(30);

list.print();