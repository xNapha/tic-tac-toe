use std::io;
use std::io::Error;

use crate::board::GameBoard;

use crate::cell::StateKind;

pub enum PlayerTurnKind {
    InvalidMove,
    ValidMove,
    // ExitGame,
    // RestartGame,
}

pub struct Coordinates {
    pub posX: usize,
    pub posY: usize,
}

pub fn placePiece(board: &mut GameBoard, player: bool) -> PlayerTurnKind {
    println!("Place piece at (e.g. A:2 or b:1):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if !input.contains(":") {
        println!("Please seperate the coordinates with a ':'");
        return PlayerTurnKind::InvalidMove;
    }

    let input: Vec<&str> = input.trim().split(":").collect();

    let input = Coordinates {
        posX: match input[1] {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            other => {
                println!("invalid x coordinate");
                println!(
                    "you entered: {}, please keep it within the range of 1-3",
                    other
                );
                return PlayerTurnKind::InvalidMove;
            }
        },
        posY: match input[0] {
            "a" | "A" => 0,
            "b" | "B" => 1,
            "c" | "C" => 2,
            other => {
                println!("invalid y coordinate");
                println!(
                    "you entered: {}, please keep it within the range of A-C",
                    other
                );
                return PlayerTurnKind::InvalidMove;
            }
        },
    };

    match &board.state[input.posX][input.posY].state {
        StateKind::Noughts => {
            println!("Cell has already been filled, please try again");
            return PlayerTurnKind::InvalidMove;
        }
        StateKind::Crosses => {
            println!("Cell has already been filled, please try again");
            return PlayerTurnKind::InvalidMove;
        }
        StateKind::Empty => {
            if input.posX == 4 || input.posY == 4 {
                return PlayerTurnKind::InvalidMove;
            } else if player {
                board.state[input.posX][input.posY].state = StateKind::Crosses;
                board.state[input.posX][input.posY].display = 'X';
            } else {
                board.state[input.posX][input.posY].state = StateKind::Noughts;
                board.state[input.posX][input.posY].display = 'O';
            }
        }
    }

    return PlayerTurnKind::ValidMove;
}
