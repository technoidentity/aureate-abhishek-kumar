struct Task {
    title : String,
    completed : bool,
}

fn main() {
    println!("Hello, world!");
    let tasks : Vec<String> = Vec::new();
    let task = Task {
        title : String::from("Learn Rust"),
        completed : false,
    };
    println!("{}", &task.title);
    println!("{}", &task.completed);
    // This is a immutable vector 
    // Immutable vectors are also possible to create 
    //The vectors can be read by this :
    println!("Number of tasks: {}", tasks.len());
    for task in &tasks {
        println!("{}", task);
    }
    let numbers = vec![1,2,3];
    println!("{:?}", numbers);
    for i in &numbers {
        print!("{i} ");
    }


}


// println!("{}", value);  // normal display
// println!("{value}");     // shorter normal display
// println!("{:?}", value); // debug display
// println!("{:#?}", value); // multi-line debug display