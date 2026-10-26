mod board;
mod game;
mod game_manager;
mod player;

use std::collections::VecDeque;

use game_manager::GameManager;
use player::Player;

fn main() {
    let players = VecDeque::from([
        Player::new("Abhishek".to_string(), 'X'),
        Player::new("Player 2".to_string(), 'O'),
    ]);

    let mut game_manager = GameManager::new(3, players);
    game_manager.play_game();
}
