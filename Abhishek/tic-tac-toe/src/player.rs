pub struct Player {
    pub name: String,
    pub symbol: char,
}

impl Player{
    pub fn new(name: String, symbol: char)-> Self{
        return Self{
            name,
            symbol,
        }

    }
    pub fn print_name(&self){
        println!("{}",self.name);
    }
    pub fn print_symbol(&self){
        println!("{}",self.symbol);
    }
}


// fn main{
//     let player1  = Player::new(
//         String::from("Abhishek"),
//         'X'
//     );
//     player1.print_name();
//     player1.print_symbol();
// }
