mod board;
mod cell;
mod player;
#[cfg(test)]
mod test;

use crate::board::*;
use crate::player::*;
fn main() {
    'application_start: loop {
        let mut board = GameBoard::new();
        'curr_game: loop {
            let current_scores = board.scores();
            println!(" |P1: {}|", current_scores.crosses_points());
            println!(" |P2: {}|", current_scores.noughts_points());
            let mut counter = 9;
            'curr_round: loop {
                println!("{}", board.display());
                let is_valid = place_piece(&mut board);

                match is_valid {
                    PlayerTurnKind::InvalidMove => continue,
                    PlayerTurnKind::ExitGame => break 'application_start,
                    PlayerTurnKind::RestartBoard => break 'curr_round,
                    PlayerTurnKind::ResetGame => continue 'application_start,
                    _ => {
                        board.set_other_player();
                        counter -= 1;
                    }
                }

                if board.check_win() {
                    println!("{}", board.display());
                    board.increase_score(&board.current_player());
                    println!("Congrats on the win");
                    break 'curr_round;
                }

                if counter == 0 {
                    println!("Draw!");
                    break 'curr_round;
                }
            }
            board = board.new_board();
        }
    }
}
