use std::io;
use rand::Rng;


fn main() {
    loop{
        println!("Welcome to the Game");
        println!("Choose Rock, Paper, or Scissors");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("There is an error. Please try again");
        let guess = input.trim().to_lowercase();
        if guess == "quit"{
            println!("Thanks for playing and if you want to quit just press Ctrl + C")
        }
        let number = rand::thread_rng().gen_range(0 , 3);

        let moves = ["rock", "paper", "scissor"];
        let comp_moves = moves[number];
        println!("{comp_moves}");
        // match guess{
        //     comp_move =>{
        //         println!(" It is a draw ");
        //     }
        // }
        if guess == comp_moves{
            println!("It is a draw");

        }else if (guess == moves[0] && comp_moves == moves[2]) || (guess == moves[1] && comp_moves == moves[0]) || (guess == moves[2] && comp_moves == moves[1]){
            println!("Your won");
        }else{
            println!("You lost buddy");
        }

    }    

}

// Can immutable var have mutable references 
//


// let name:String = String.from("Alex");
// let firstName = name;
// will this work if yes why if no why explain 
