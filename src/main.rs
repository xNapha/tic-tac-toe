mod board;
mod cell;
mod player;

use crate::board::*;
use crate::player::*;
fn main() {
    'application_start: loop {
        let mut board = GameBoard::new();
        let mut current_player = PlayerKind::Crosses;
        'curr_game: loop {
            println!(" |P1: {}|", board.scores[0]);
            println!(" |P2: {}|", board.scores[1]);
            let mut counter = 9;
            'curr_round: loop {
                display(&board);
                let is_valid = place_piece(&mut board, &current_player);

                match is_valid {
                    PlayerTurnKind::InvalidMove => continue,
                    PlayerTurnKind::ExitGame => break 'application_start,
                    PlayerTurnKind::RestartBoard => break 'curr_round,
                    PlayerTurnKind::ResetGame => continue 'application_start,
                    _ => (),
                }

                if check_win(&board) {
                    display(&board);
                    board.increase_score(&current_player);
                    println!("Congrats on the win");
                    break 'curr_round;
                }

                current_player = switch_player(current_player);
                counter -= 1;

                if counter == 0 {
                    println!("Draw!");
                    break 'curr_round;
                }
            }
            board = board.new_board();
        }
    }
}
