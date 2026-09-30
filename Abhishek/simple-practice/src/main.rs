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

impl Person { 
    fn new(name: String, age : u32 ) -> Self {
        Self {
            name,
            age,

        }
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
    let p1 = Person::new(String::from("Abhishek"),24);
    println!("{}", p1.name);
    print_book( book);
}
