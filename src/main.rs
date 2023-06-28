mod board;
mod cell;
mod player;

use crate::board::*;
use crate::player::*;
fn main() {
    'application_start: loop {
        let mut board = GameBoard::new();
        let mut is_player_1 = true;
        let mut is_game_won = false;
        while !is_game_won {
            display(&mut board);
            let is_valid = placePiece(&mut board, is_player_1);
            if !is_valid {
                continue;
            }
            // is_game_won = checkWin(&board);
            is_game_won = true;
            is_player_1 = !is_player_1;
        }
        if is_game_won {
            break;
        }
    }
}
