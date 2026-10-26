// fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
//     if a.len() >= b.len() {
//         a
//     } else {
//         b
//     }
// }

// fn main() {
//     let first = String::from("ledger");
//     let result;
    
//     let second = String::from("idempotency");
        
//     result = longest(first.as_str(), second.as_str());
    
//     println!("longest is {}", result);
// }



use std::collections::HashMap;
use std::num::ParseIntError;

fn parse_line(line: &str) -> Result<(String, u32),ParseIntError> {
    //TODO: Implement the function to parse a line and return a Result containing a tuple of (user, score) or a ParseIntError if the score is not a valid integer.
    let part:Vec<&str> = line.split(',').collect();
    if part.len() != 2 {
        return Err("Invalid line format".parse().unwrap());     
    }
    let user = part[0].to_string();
    let score = part[1].parse::<u32>()?;
    Ok((user, score))   
}

fn totals(lines: &[&str]) -> (HashMap<String, u32>, usize) {
    //TODO: Implement this function to return a tuple containing a HashMap of user totals and the count of rejected lines.
    let mut by_user = HashMap::new();
    let mut rejected = 0;

    for line in lines {
        match parse_line(line) {
            Ok((user, score)) => {
                *by_user.entry(user).or_insert(0) += score;
            }
            Err(_) => {
                rejected += 1;
            }
        }
    }

    (by_user, rejected)
}

fn main() {
    let lines = ["kiran,3", "meera,2", "kiran,4", "arjun,x", "meera,1", "sana,"];
    let (by_user, rejected) = totals(&lines);

    let mut users: Vec<&String> = by_user.keys().collect();
    users.sort();
    for u in users {
        println!("{}: {}", u, by_user[u]);
    }
    println!("rejected: {}", rejected);
}
