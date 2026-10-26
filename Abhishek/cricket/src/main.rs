use uuid::Uuid;
use std::io;

#[derive(Debug)]
enum PlayerRole {
    Batsman,
    Bowler,
    AllRounder,
    WicketKeeper,
}

#[derive(Debug)]
struct Player { 
    id : Uuid,
    name : String,
    role : PlayerRole,
    runs : i32,
    balls_faced : i32,
    wickets : i32,
    balls_bowled : i32,
}

impl Player { 
    fn new(id : Uuid, name: &str, role : PlayerRole) -> Self { 
        Self { 
            id,
            name:String::from(name),
            role,
            runs:0,
            balls_faced:0,
            wickets:0,
            balls_bowled:0,
        }
    }
    fn add_runs(&mut self, runs: i32){
        self.runs += runs;
    }
    fn face_ball(&mut self){
        self.balls_faced += 1;
    }
    fn wickets(&mut self){
        self.wickets += 1;
    }
    fn balls_bowled(&mut self){
        self.balls_bowled += 1;
    }
    fn show_stats(&self){
        println!("{} - Runs : {} , Balls : {} , Bowled: {} , Wickets: {}", 
                    self.name,
                    self.runs,
                    self.balls_faced,
                    self.balls_bowled,
                    self.wickets,
                    );
    }
}
#[derive(Debug)]
struct Team { 
    id: Uuid,
    name : String,
    players: Vec<Player>
}

impl Team{
    fn new(name:&str) -> Self {
        Self { 
            id: Uuid::new_v4(),
            name:String::from(name),
            players: Vec::new(),
        }
    }
        fn add_player(&mut self, player:Player){
            self.players.push(player);
        }
        fn get_player(&self, player_id : Uuid) -> Option<&Player>{
            self.players
                .iter()
                .find(|player| player.id == player_id) 
        }
        fn remove_player(&mut self, player_id: Uuid){
            self.players.retain(|player| player.id != player_id)
        }
        fn show_team(&self){
            println!("\nTeam: {}" , self.name);

            for player in &self.players{
                println!(
                    "{} : {:?}", player.name, player.role
                );
            }
        }
    
}

#[derive(Debug)]
struct Match { 
    id: Uuid,
    team1 : Team,
    team2 : Team,

    team1_score: i32,
    team2_score: i32,

    team1_wickets: i32,
    team2_wickets: i32,
    
}

impl Match{
    fn new (team1: Team, team2 : Team)->Self{
        Self {
            id: Uuid::new_v4(), 
            
            team1,
            team2,

            team1_score:0,
            team2_score:0,
            
            team1_wickets: 0,
            team2_wickets: 0,
            
        }
    }
    fn add_team1_runs(&mut self, runs: i32){
        self.team1_score += runs;
    }
    fn add_team2_runs(&mut self, runs: i32){
        self.team2_score += runs;
    }
    fn add_team1_wickets(&mut self){
        self.team1_wickets += 1;
    }
    fn add_team2_wickets(&mut self){
        self.team2_wickets += 1;
    }
    fn show_score(&self){
        println!(
            "\n{}: {}/{}",self.team1.name, self.team1_score, self.team1_wickets
        );
        println!(
            "\n{}: {}/{}",self.team2.name, self.team2_score, self.team2_wickets
        );
    }   
}

fn get_input()->String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    input.trim().to_string()
}
fn main () {
    // println!({}, "Hello World!")'
    println!("Enter team name: ");
    
    let name = get_input();
}