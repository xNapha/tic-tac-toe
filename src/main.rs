mod board;
mod cell;
mod player;

use crate::board::*;
use crate::player::*;
fn main() {
    'application_start: loop {
        let mut board = GameBoard::new();
        let mut player_type = PlayerKind::Crosses;
        let mut is_game_won = false;
        let mut counter = 9;
        while !is_game_won {
            display(&mut board);
            let is_valid = place_piece(&mut board, &player_type);
            match is_valid {
                PlayerTurnKind::InvalidMove => continue,
                PlayerTurnKind::ExitGame => {
                    break 'application_start;
                }
                PlayerTurnKind::RestartGame => {
                    break;
                }
                _ => (),
            }
            is_game_won = check_win(&board);
            player_type = match player_type {
                PlayerKind::Crosses => PlayerKind::Noughts,
                PlayerKind::Noughts => PlayerKind::Crosses,
            };
            counter -= 1;
            if counter == 0 && !is_game_won {
                println!("Draw!");
            }
        }
        if is_game_won {
            display(&mut board);
            println!("Congrats on the win");
            break;
        }
    }
}
