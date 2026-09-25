use std::collections::VecDeque;
use std::io::{self, Write};

use crate::board::Board;
use crate::game::Game;
use crate::player::Player;

pub struct GameManager {
    pub game: Game,
    pub moves: usize,
}

impl GameManager {
    pub fn new(board_size: usize, players: VecDeque<Player>) -> Self {
        assert!(players.len() >= 2, "Tic-tac-toe needs at least two players");

        Self {
            game: Game::new(players, Board::new(board_size)),
            moves: 0,
        }
    }

    pub fn play_game(&mut self) {
        println!("Enter a row and column from 1 to {}.", self.game.board.size);

        while self.game.status == "IN_PROGRESS" {
            println!("\n{}", self.game.board);

            let player = self.game.players.front().expect("Game needs a player");
            let name = player.name.clone();
            let symbol = player.symbol;

            let Some((row, col)) = self.get_player_input(&name, symbol) else {
                continue;
            };

            if !self.game.board.insert_new_symbol(row, col, symbol) {
                continue;
            }

            self.moves += 1;

            if self.game.board.has_winner(symbol) {
                println!("\n{}", self.game.board);
                println!("{name} wins!");
                self.game.status = "COMPLETE".to_string();
            } else if self.moves == self.game.board.size * self.game.board.size {
                println!("\n{}", self.game.board);
                println!("The game is a draw.");
                self.game.status = "COMPLETE".to_string();
            } else {
                self.game.players.rotate_left(1);
            }
        }
    }

    fn get_player_input(&self, name: &str, symbol: char) -> Option<(usize, usize)> {
        print!("{name} ({symbol}), enter row and column: ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        let values: Vec<&str> = input.split_whitespace().collect();
        if values.len() != 2 {
            println!("Enter exactly two numbers, for example: 2 3");
            return None;
        }

        let row = values[0].parse::<usize>().ok()?;
        let col = values[1].parse::<usize>().ok()?;
        if row == 0 || col == 0 {
            println!("Row and column start at 1");
            return None;
        }

        Some((row - 1, col - 1))
    }
}
