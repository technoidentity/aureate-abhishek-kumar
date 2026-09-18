fn main() {
    println!("Hello, world!");
    let tasks : Vec<String> = Vec::new();
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
        println!("{i}");
    }
}


// println!("{}", value);  // normal display
// println!("{value}");     // shorter normal display
// println!("{:?}", value); // debug display
// println!("{:#?}", value); // multi-line debug display