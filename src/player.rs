use std::io;

use crate::board::GameBoard;

use crate::cell::StateKind;

#[derive(Copy, Clone)]
pub enum PlayerKind {
    Noughts,
    Crosses,
}

pub enum PlayerTurnKind {
    InvalidMove,
    ValidMove,
    ExitGame,
    RestartBoard,
    ResetGame,
}

pub struct Coordinates {
    pub pos_x: usize,
    pub pos_y: usize,
}

pub fn place_piece(board: &mut GameBoard) -> PlayerTurnKind {
    let player = board.current_player();
    println!("Place piece at (e.g. A:2 or b:1):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.to_lowercase().contains("exit") {
        println!("Have a good day.");
        return PlayerTurnKind::ExitGame;
    } else if input.to_lowercase().contains("reset") {
        println!("Resetting scores.");
        return PlayerTurnKind::ResetGame;
    } else if input.to_lowercase().contains("restart") {
        println!("Restarting...");
        return PlayerTurnKind::RestartBoard;
    } else if !input.contains(":") {
        println!("Please seperate the coordinates with a ':'");
        return PlayerTurnKind::InvalidMove;
    };

    let input: Vec<&str> = input.trim().split(":").collect();

    let input = new_coordinates_struct(input);

    if input.pos_x == 9 || input.pos_y == 9 {
        return PlayerTurnKind::InvalidMove;
    };

    check_cell_state(board, input, player)
}

fn new_coordinates_struct(input: Vec<&str>) -> Coordinates {
    Coordinates {
        pos_x: match input[1] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            other => {
                println!("invalid x coordinate");
                println!(
                    "you entered: {}, please keep it within the range of 1-3",
                    other
                );
                9
            }
        },
        pos_y: match input[0].to_lowercase().as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            other => {
                println!("invalid y coordinate");
                println!(
                    "you entered: {}, please keep it within the range of A-C",
                    other
                );
                9
            }
        },
    }
}

fn check_cell_state(
    board: &mut GameBoard,
    input: Coordinates,
    player: PlayerKind,
) -> PlayerTurnKind {
    let x = input.pos_x;
    let y = input.pos_y;
    let current_cell = &board.state()[x][y].state();
    match current_cell {
        StateKind::Empty => match player {
            PlayerKind::Crosses => {
                let mut new_cell = board.state()[x][y];
                new_cell.update_cell(StateKind::Crosses, 'X');
                board.set_cell(x, y, new_cell);
                return PlayerTurnKind::ValidMove;
            }
            PlayerKind::Noughts => {
                let mut new_cell = board.state()[x][y];
                new_cell.update_cell(StateKind::Noughts, 'O');
                board.set_cell(x, y, new_cell);
                return PlayerTurnKind::ValidMove;
            }
        },
        _ => {
            println!("That cell has already been filled, please try again");
            return PlayerTurnKind::InvalidMove;
        }
    }
}
