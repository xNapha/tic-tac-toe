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
        let mut counter = 9;
        while !is_game_won {
            display(&mut board);
            println!("is player 1: {}", is_player_1);
            let is_valid = placePiece(&mut board, is_player_1);
            match is_valid {
                PlayerTurnKind::InvalidMove => continue,
                PlayerTurnKind::ValidMove => (),
            }
            is_game_won = checkWin(&board);
            // is_game_won = true;
            is_player_1 = !is_player_1;
            counter -= 1;
            if counter == 0 && !is_game_won {
                println!("Draw!");
            }
        }
        if is_game_won {
            break;
        }
    }
}
