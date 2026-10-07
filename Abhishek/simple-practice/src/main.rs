struct Student { 
    name: String,
    age: i32,
}

struct Book {
    title : String,
    pages : u32,
}

struct Person {
    name : String,
    age : u32,

}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Command { 
    Add(String),
    Remove(usize),
    List,
}

impl Person { 
    fn new(name: String, age : u32 ) -> Self {
        Self {
            name,
            age,

        }
    }
    fn print_name(&self, name: String) {
        println!("{}", self.name);
    }
}



fn print_book(book: Book ){
    println!("{} has {} pages", book.title, book.pages);
}


fn main() {
    let student = Student {
        name: String::from("Sanchay"),
        age: 22,
    };
    let book = Book {
        title: String::from("Oreilly - Rust"),
        pages: 500,
    };
    let light = TrafficLight::Red;
    
    match light {
        TrafficLight::Red => println("Stop"),
        TrafficLight::Yellow => println("Wait"),
        TrafficLight::Red => println("Go"),
    }
    let command = Command::Add(String::from("Milk"));
    match command {
        Command::Add(item) => {println!("Adding item {}", item);
        }
        Command::Remove(index) => {println!("Removing item number {}", index);
        }
        Command::List => {
            println!("Listing Items");
        }
    }
    let p1 = Person::new(String::from("Abhishek"),24);
    println!("{}", p1.name);
    p1.print_name();
    print_book( book);
}
