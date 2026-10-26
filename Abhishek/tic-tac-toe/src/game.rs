use std::collections::VecDeque;

use crate::board::Board;
use crate::player::Player;

pub struct Game {
    pub status: String,
    pub players: VecDeque<Player>,
    pub board: Board,
}

impl Game {
    pub fn new(players: VecDeque<Player>, board: Board) -> Self {
        Self {
            players,
            board,
            status: "IN_PROGRESS".to_string(),
        }
    }
}
