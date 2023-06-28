use std::io;
use std::io::Error;

use crate::board::GameBoard;

pub enum PlayerKind {
    Player1,
    Player2,
}

pub struct Coordinates {
    posX: Option<u32>,
    posY: Option<u32>,
}
pub struct Player {
    pub player: PlayerKind,
}

pub fn placePiece(board: &mut GameBoard, player: bool) -> bool {
    println!("Place piece at (e.g. A:2 or a:2):");
    let mut is_valid = true;
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: Vec<&str> = input.trim().split(":").collect();

    let input = Coordinates {
        posX: match input[0] {
            "a" | "A" => Some(0),
            "b" | "B" => Some(1),
            "c" | "C" => Some(2),
            other => {
                println!("invalid x coordinate");
                println!(
                    "you entered: {}, please keep it within the change of A-C",
                    other
                );
                is_valid = false;
                None
            }
        },
        posY: match input[1] {
            "1" => Some(0),
            "2" => Some(1),
            "3" => Some(2),
            _ => {
                println!("invalid y coordinate");
                is_valid = false;
                None
            }
        },
    };
    println!("posX: {:#?}, posY: {:#?}", input.posX, input.posY);
    is_valid
}
